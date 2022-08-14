const { readFileSync } = require('fs');
const { resolve } = require('path');

const TEST_FUNCTION_NAME = 'test';

const DIFF_JSJOULES_FILE_NAME = resolve(`${__dirname}/diff-jsjoules-instr.js`);
const DIFF_JSJOULES_NAME = 'diff_jsjoules';
const DIFF_JSJOULES_START_FUNCTION_NAME = 'start'
const DIFF_JSJOULES_STOP_FUNCTION_NAME = 'stop'

function buildCall(jscodeshift, function_name, test_identifier) {
    return jscodeshift.awaitStatement(
        jscodeshift.callExpression(
            jscodeshift.memberExpression(
                jscodeshift.identifier(DIFF_JSJOULES_NAME),
                jscodeshift.identifier(function_name)
            ),
            [jscodeshift.literal(test_identifier)]
        )
    );
}

module.exports =
    function (fileInfo, api, options) {
        const jscodeshift = api.jscodeshift;
        const test_selection = JSON.parse(readFileSync(options.tests));
        const source = jscodeshift(fileInfo.source);
        let one_test_has_been_instrumented = false;
        source
            .find(jscodeshift.ExpressionStatement)
            .filter(statement => statement.value.expression.callee !== undefined && statement.value.expression.callee.name === TEST_FUNCTION_NAME)
            .forEach((test_function) => {
                const test_identifier = test_function.value.expression.arguments.filter(args => args.type === 'Literal')[0].value;
                if (test_selection.test_selection.indexOf(test_identifier) >= 0) {
                    one_test_has_been_instrumented = true;
                    console.log(`Instrumenting ${test_identifier}`);
                    test_function.value.expression.arguments
                        .filter(args => args.type == jscodeshift.ArrowFunctionExpression)
                        .forEach((arrowFunctionExpression) => {
                            arrowFunctionExpression.async = true;
                            const test_identifier_snaked_case = test_identifier.replaceAll(' ', '_');
                            const startCall = buildCall(jscodeshift, DIFF_JSJOULES_START_FUNCTION_NAME, test_identifier_snaked_case)
                            arrowFunctionExpression.body.body.unshift(startCall);
                            const stopCall = buildCall(jscodeshift, DIFF_JSJOULES_STOP_FUNCTION_NAME, test_identifier_snaked_case);
                            arrowFunctionExpression.body.body.push(stopCall);
                        });
                }
            });
        if (one_test_has_been_instrumented) {
            const import_diff_jsjoules = jscodeshift.variableDeclaration('const',
                [jscodeshift.variableDeclarator(
                    jscodeshift.identifier(DIFF_JSJOULES_NAME),
                    jscodeshift.callExpression(
                        jscodeshift.identifier('require'),
                        [jscodeshift.literal(DIFF_JSJOULES_FILE_NAME)]
                    )
                )]
            );
            source.__paths[0].value.program.body.unshift(import_diff_jsjoules);
        }
        return source.toSource();
    };