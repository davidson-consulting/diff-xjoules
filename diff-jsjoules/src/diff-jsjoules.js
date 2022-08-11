const { readFileSync, writeFileSync } = require('fs');
const { resolve } = require('path');

/**
 * Executes a shell command and return it as a Promise.
 * @param cmd {string}
 * @return {Promise<string>}
 */
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
    return path.endsWith("/") ? path.slice(0, -1) : path;
}

function compute_coverage(absolute_path_project_v1, coverage_output_json) {
    const coverages = [];
    for (var sourceFile in coverage_output_json.coverageMap) {
        const coverateStatementMap = coverage_output_json.coverageMap[sourceFile].s;
        const coverage = {}
        coverage.filename = sourceFile.substring(absolute_path_project_v1.length + 1);
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

async function coverage_task(project_path_v1, project_path_v2, diff_path_file, output_folder_path) {
    // TODO we could here use --onChanged command line option of jest to rely on Jest for narrowing the test selection
    // TODO we could here run test per test to have their coverage individually, it would be an accurate mode but much more slower and expensive
    const modified_files = get_modified_files_from_diff(project_path_v1, diff_path_file);
    const coverage_output_json = JSON.parse(await exec_command(['jest', '--coverage', '--json', '--findRelatedTests', modified_files.join(' ')].join(' '), project_path_v1));
    const coverages = compute_coverage(resolve(project_path_v1), coverage_output_json);
    const testCoverages = compute_test_coverage(coverage_output_json, coverages);
    writeFileSync(output_folder_path + '/coverage_v1.json', JSON.stringify(testCoverages, undefined, 4));
    writeFileSync(output_folder_path + '/coverage_v2.json', JSON.stringify(testCoverages, undefined, 4));
}

async function main(project_path_v1, project_path_v2, diff_path_file, output_folder_path) {
    project_path_v1 = sanitize_slash(project_path_v1);
    project_path_v2 = sanitize_slash(project_path_v2);
    output_folder_path = sanitize_slash(output_folder_path);
    coverage_task(project_path_v1, project_path_v2, diff_path_file, output_folder_path);
}

main('test_resources/diff-jsjoules-toy-nodejs-project', 'test_resources/diff-jsjoules-toy-nodejs-project-v2', 'test_resources/diff', 'target'); 

exports.coverage_task = coverage_task;
exports.get_modified_files_from_diff = get_modified_files_from_diff;
exports.sanitize_slash = sanitize_slash;