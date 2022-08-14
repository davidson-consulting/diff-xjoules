const { readFileSync, writeFileSync, mkdirSync, existsSync } = require('fs');
const { resolve } = require('path');
const yargs = require('yargs');
const dgram = require('dgram');
const { Buffer } = require('buffer');

function exec_command(cmd, cwd) {
    console.log(cmd);
    const exec = require('child_process').exec;
    return new Promise((resolve, reject) => {
        exec(cmd, { cwd: cwd }, (error, stdout, stderr) => {
            if (error) {
                console.warn(error);
            }
            resolve(stdout ? stdout : stderr);
        });
    });
}

function get_modified_files_from_diff(prefix_project, diff_path_file) {
    const diff_content = readFileSync(diff_path_file, 'utf-8');
    const splitted_diff_content = diff_content.split('\n');
    const modified_files = []
    for (let i = 0; i < splitted_diff_content.length; i++) {
        if (splitted_diff_content[i].startsWith('diff -r')) {
            let modified_file = splitted_diff_content[i].split(' ')[2];
            if (modified_file.startsWith(prefix_project)) {
                modified_file = modified_file.substring(prefix_project.length + 1);
            }
            modified_files.push(modified_file);
        }
    }
    return modified_files;
}

function sanitize(path) {
    return path !== undefined && path.endsWith("/") ? path.slice(0, -1) : path;
}

function compute_coverage(absolute_path_project, coverage_output_json) {
    const coverages = [];
    for (var sourceFile in coverage_output_json.coverageMap) {
        const statementMap = coverage_output_json.coverageMap[sourceFile].statementMap;
        const coverageStatementMap = coverage_output_json.coverageMap[sourceFile].s;
        const coverage = {}
        coverage.filename = sourceFile.substring(absolute_path_project.length + 1);
        coverage.covered_lines = [];
        for (var coveredStatement in coverageStatementMap) {
            if (coverageStatementMap[coveredStatement] > 0) {
                coverage.covered_lines.push(+statementMap[coveredStatement].start.line);
            }
        }
        coverages.push(coverage);
    }
    return coverages;
}

function compute_test_coverage(coverage_output_json, coverages) {
    const test_coverages = {};
    test_coverages.test_coverages = [];
    coverage_output_json.testResults.map(test_result => {
        test_result.assertionResults.forEach(assertion_result => {
            var test_coverage = {};
            test_coverage.test_identifier = assertion_result.fullName;
            test_coverage.file_coverages = [];
            coverages.forEach(coverage => test_coverage.file_coverages.push(coverage));
            test_coverages.test_coverages.push(test_coverage);
        })
    });
    return test_coverages;
}

async function coverage_task(project_path_v1, diff_path_file, output_folder_path) {
    // TODO we could here use --onChanged command line option of jest to rely on Jest for narrowing the test selection
    // TODO we could here run test per test to have their coverage individually, it would be an accurate mode but much more slower and expensive
    const modified_files = get_modified_files_from_diff(project_path_v1, diff_path_file);
    const coverage_output_json = JSON.parse(await exec_command(['jest', '--coverage', '--json', '--findRelatedTests', modified_files.join(' ')].join(' '), project_path_v1));
    const coverages = compute_coverage(resolve(project_path_v1), coverage_output_json);
    const testCoverages = compute_test_coverage(coverage_output_json, coverages);
    writeFileSync(output_folder_path + '/coverage_v1.json', JSON.stringify(testCoverages, undefined, 4));
    writeFileSync(output_folder_path + '/coverage_v2.json', JSON.stringify(testCoverages, undefined, 4));
}

async function instrumentation_task(project_path_v1, project_path_v2, json_test_path) {
    await exec_command(['jscodeshift', '-t', `${__dirname}/instrumentation.js`, `${project_path_v1}/src`, `--tests=${json_test_path}`].join(' '));
    await exec_command(['jscodeshift', '-t', `${__dirname}/instrumentation.js`, `${project_path_v2}/src`, `--tests=${json_test_path}`].join(' '));
}

async function send(project_path) {
    const client = dgram.createSocket('udp4');
    return new Promise((resolve, reject) => {
        client.send(Buffer.from(`report ${project_path}/diff-measurements/measurements.json`), 2000, () => {
            console.log(`report ${project_path}/diff-measurements/measurements.json sent`)
            resolve(client.close());
        });
    });
}

function convert_tlpc_report_to_diff_xjoules_report(absolute_project_path) {
    const tlpc_report = JSON.parse(readFileSync(`${absolute_project_path}/diff-measurements/measurements.json`));
    const diff_xjoules_report = {}
    diff_xjoules_report.test_measures = [];
    for (let test_identifier in tlpc_report) {
        const test_measures = {};
        test_measures.test_identifier = test_identifier.replaceAll('_', ' ');
        const measure = []
        for (let indicator in tlpc_report[test_identifier]) {
            const indicator_measure = {}
            indicator_measure.indicator = indicator;
            indicator_measure.value = tlpc_report[test_identifier][indicator];
            measure.push(indicator_measure);
        }
        test_measures.measures = [measure];
        diff_xjoules_report.test_measures.push(test_measures);
    }

    writeFileSync(`${absolute_project_path}/diff-measurements/measurements.json`, JSON.stringify(diff_xjoules_report, undefined, 4));
}

async function execution_task(project_path_v1, project_path_v2, json_test_path) {
    const tests_to_run = JSON.parse(readFileSync(json_test_path)).test_selection.join('|');
    const absolute_project_path_v1 = resolve(project_path_v1);
    const absolute_project_path_v2 = resolve(project_path_v2);
    if (!existsSync(`${absolute_project_path_v1}/diff-measurements`)) {
        mkdirSync(`${absolute_project_path_v1}/diff-measurements`);
        mkdirSync(`${absolute_project_path_v2}/diff-measurements`);
    }
    await exec_command(['jest', '-t', `"${tests_to_run}"`, '--runInBand'].join(' '), absolute_project_path_v1);
    await send(absolute_project_path_v1);
    convert_tlpc_report_to_diff_xjoules_report(absolute_project_path_v1);
    await exec_command(['jest', '-t', `"${tests_to_run}"`, '--runInBand'].join(' '), absolute_project_path_v2);
    await send(absolute_project_path_v2);
    convert_tlpc_report_to_diff_xjoules_report(absolute_project_path_v2);
}

async function main(args) {
    if (args.help || args.version) {
        return;
    }
    const project_path_v1 = sanitize(args.pathToProjectV1);
    const project_path_v2 = sanitize(args.pathToProjectV2);
    const output_folder_path = sanitize(args.outputPath);
    const json_test_path = args.tests;
    const path_diff_file = args.pathDiffFile;
    if (argv._.includes('coverage')) {
        coverage_task(project_path_v1, path_diff_file, output_folder_path);
    } else if (argv._.includes('instrumentation')) {
        instrumentation_task(project_path_v1, project_path_v2, json_test_path);
    } else if (argv._.includes('execution')) {
        execution_task(project_path_v1, project_path_v2, json_test_path);
    }
}

const argv = yargs
    .command('coverage', 'Perform the coverage task', {
        accurate: {
            description: 'Enable accurate mode which may results with more precise test selection but slower execution.',
            default: false,
            type: 'boolean'
        },
        onChanges: {
            description: 'Use --onChanges command line option from jest to compute test to run rather than using --findRelatedTests, requiring to be in a local git repository.',
            default: false,
            type: 'boolean'
        }
    })
    .command('instrumentation', 'Perform the instrumentation task')
    .command('execution', 'Perform the execution task')
    .option('path-to-project-v1', {
        alias: 'f',
        description: 'Path to the program in the first version.',
        type: 'string'
    })
    .option('path-to-project-v2', {
        alias: 's',
        description: 'Path to the program in the second version.',
        type: 'string'
    })
    .option('tests', {
        alias: 't',
        description: 'Path to the json file of tests set.',
        type: 'string'
    })
    .option('output-path', {
        alias: 'o',
        description: 'Path to the output.',
        type: 'string'
    })
    .option('path-diff-file', {
        alias: 'd',
        description: 'Path to the file containing the UNIX diff.',
        type: 'string'
    })
    .help()
    .alias('help', 'h').argv;

if (require.main === module) {
    main(argv);
}

exports.coverage_task = coverage_task;
exports.get_modified_files_from_diff = get_modified_files_from_diff;
exports.sanitize = sanitize;
exports.compute_coverage = compute_coverage;
exports.compute_test_coverage = compute_test_coverage;
exports.exec_command = exec_command;
exports.instrumentation_task = instrumentation_task;
exports.execution_task = execution_task;