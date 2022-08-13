const { readFileSync, writeFileSync } = require('fs');
const { resolve } = require('path');
const yargs = require('yargs');
const instrumentation = require('./instrumentation');

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

function sanitize_slash(path) {
    return path !== undefined && path.endsWith("/") ? path.slice(0, -1) : path;
}

function compute_coverage(absolute_path_project, coverage_output_json) {
    const coverages = [];
    for (var sourceFile in coverage_output_json.coverageMap) {
        const coverateStatementMap = coverage_output_json.coverageMap[sourceFile].s;
        const coverage = {}
        coverage.filename = sourceFile.substring(absolute_path_project.length + 1);
        coverage.covered_lines = [];
        for (var coveredLine in coverateStatementMap) {
            if (coverateStatementMap[coveredLine] > 0) {
                coverage.covered_lines.push(+coveredLine);
            }
        }
        coverages.push(coverage);
    }
    return coverages;
}

function compute_test_coverage(coverage_output_json, coverages) {
    const testCoverages = []
    coverage_output_json.testResults.map(testResult => {
        testResult.assertionResults.forEach(assertionResult => {
            var testCoverage = {};
            testCoverage.test_identifier = assertionResult.fullName;
            testCoverage.file_coverages = [];
            coverages.forEach(coverage => testCoverage.file_coverages.push(coverage));
            testCoverages.push(testCoverage);
        })
    });
    return testCoverages;
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
    await exec_command(['jscodeshift', '-t', 'src/instrumentation.js', `${project_path_v1}/src`, `--tests=${json_test_path}`].join(' '));
    await exec_command(['jscodeshift', '-t', 'src/instrumentation.js', `${project_path_v2}/src`, `--tests=${json_test_path}`].join(' '));
}

async function main(args) {
    if (args.help || args.version) {
        return;
    }
    const project_path_v1 = sanitize_slash(args.pathToProjectV1);
    const project_path_v2 = sanitize_slash(args.pathToProjectV2);
    const output_folder_path = sanitize_slash(args.outputPath);
    const json_test_path = args.tests;
    const path_diff_file = args.pathDiffFile;
    if (argv._.includes('coverage')) {
        coverage_task(project_path_v1, path_diff_file, output_folder_path);
    } else if (argv._.includes('instrumentation')) {
        instrumentation_task(project_path_v1, project_path_v2, json_test_path);
    } else if (argv._.includes('execution')) {
        console.log('execution');
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
exports.sanitize_slash = sanitize_slash;
exports.compute_coverage = compute_coverage;
exports.compute_test_coverage = compute_test_coverage;
exports.exec_command = exec_command;
