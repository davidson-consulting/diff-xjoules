const { readFileSync } = require('fs')
const { resolve } = require('path')

const TEST_FUNCTION_NAME = 'test'
const IT_FUNCTION_NAME = 'it'
const DESCRIBE_FUNCTION_NAME = 'describe'

const DIFF_JSJOULES_FILE_NAME = resolve(`${__dirname}/diff-jsjoules-instr.js`)
const DIFF_JSJOULES_NAME = 'diff_jsjoules'
const DIFF_JSJOULES_START_FUNCTION_NAME = 'start'
const DIFF_JSJOULES_STOP_FUNCTION_NAME = 'stop'

function match_describe (candidate) {
  return candidate.value.callee !== undefined && candidate.value.callee.name === DESCRIBE_FUNCTION_NAME
}

function buildCall (jscodeshift, function_name, test_identifier) {
  return jscodeshift.awaitStatement(
    jscodeshift.callExpression(
      jscodeshift.memberExpression(
        jscodeshift.identifier(DIFF_JSJOULES_NAME),
        jscodeshift.identifier(function_name)
      ),
      [jscodeshift.literal(test_identifier)]
    )
  )
}

function find_describe_parent (startPoint) {
  let current = startPoint.parentPath
  while (!match_describe(current)) {
    if (current === undefined || current.parentPath === undefined || current.parentPath === null) {
      return undefined
    } else {
      current = current.parentPath
    }
  }
  return current
}

function gather_describes_from_statement (starting_point) {
  let current = starting_point
  const describes = []
  while (current !== undefined) {
    current = find_describe_parent(current)
    if (current !== undefined) {
      describes.push(current.value.arguments[0].value)
    }
  }
  describes.reverse()
  return describes.join(' ')
}

module.exports =
  function (fileInfo, api, options) {
    const jscodeshift = api.jscodeshift
    const test_selection = JSON.parse(readFileSync(options.tests))
    const source = jscodeshift(fileInfo.source)
    let one_test_has_been_instrumented = false
    source
      .find(jscodeshift.ExpressionStatement)
      .filter(statement => statement.value !== undefined)
      .filter(statement => statement.value.expression.callee !== undefined && (statement.value.expression.callee.name === TEST_FUNCTION_NAME || statement.value.expression.callee.name === IT_FUNCTION_NAME))
      .filter(statement => statement.value.expression.arguments.filter(args => args.type === 'Literal')[0] !== undefined)
      .forEach(statement => {
        const base_test_identifier = statement.value.expression.arguments.filter(args => args.type === 'Literal')[0].value
        const describes = gather_describes_from_statement(statement)
        let test_identifier
        if (describes.length > 0) {
          test_identifier = [describes, base_test_identifier].join(' ')
        } else {
          test_identifier = base_test_identifier
        }
        if (test_selection.test_selection.indexOf(test_identifier) >= 0) {
          one_test_has_been_instrumented = true
          console.log(`Instrumenting ${test_identifier}`)
          const test_function = statement.value.expression.arguments[1]
          test_function.async = true
          const test_identifier_snaked_case = test_identifier.replaceAll(' ', '_').replaceAll('"', '-')
          const startCall = buildCall(jscodeshift, DIFF_JSJOULES_START_FUNCTION_NAME, test_identifier_snaked_case)
          test_function.body.body.unshift(startCall)
          const stopCall = buildCall(jscodeshift, DIFF_JSJOULES_STOP_FUNCTION_NAME, test_identifier_snaked_case)
          test_function.body.body.push(stopCall)
        }
      })
    if (one_test_has_been_instrumented) {
      const import_diff_jsjoules = jscodeshift.variableDeclaration('const',
        [jscodeshift.variableDeclarator(
          jscodeshift.identifier(DIFF_JSJOULES_NAME),
          jscodeshift.callExpression(
            jscodeshift.identifier('require'),
            [jscodeshift.literal(DIFF_JSJOULES_FILE_NAME)]
          )
        )]
      )
      source.__paths[0].value.program.body.unshift(import_diff_jsjoules)
    }
    return source.toSource()
  }
