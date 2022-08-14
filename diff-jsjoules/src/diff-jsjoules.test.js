const diff_jsjoules = require('./diff-jsjoules');

const { readFileSync, unlinkSync, existsSync } = require('fs');

test('test exec_command', async () => {
    const stdout_result = await diff_jsjoules.exec_command('ls -a');
    expect(stdout_result).toContain('src');
});

test('test get_modified_files_from_diff', () => {
    const modified_files = diff_jsjoules.get_modified_files_from_diff('test_resources/diff-jsjoules-toy-nodejs-project', 'test_resources/diff');
    expect(modified_files.length).toBe(1);
    expect(modified_files[0]).toBe('src/app.js');
});

test('test sanitize', () => {
    expect(diff_jsjoules.sanitize('path/with/end/slash/')).toBe('path/with/end/slash');
    expect(diff_jsjoules.sanitize('path/without/end/slash')).toBe('path/without/end/slash');
    expect(diff_jsjoules.sanitize(undefined)).toBe(undefined);
});

test('test compute_coverage', () => {
    const coverage_output_json = JSON.parse(readFileSync('test_resources/coverage_jest.json'));
    const coverage = diff_jsjoules.compute_coverage('diff-jsjoules-toy-nodejs-project', coverage_output_json);
    expect(coverage.length).toBe(1);
    expect(coverage[0].filename).toBe('src/app.js');
    expect(coverage[0].covered_lines.length).toBe(41);
});

test('test compute_test_coverage', () => {
    const coverages = [
        {
            filename: 'src/app.js',
            covered_lines: [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
                10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
                30, 39, 40, 41, 42, 43, 44, 45, 46, 47,
                48
            ]
        }
    ];
    const coverage_output_json = JSON.parse(readFileSync('test_resources/coverage_jest.json'));
    const test_coverage = diff_jsjoules.compute_test_coverage(coverage_output_json, coverages);
    expect(test_coverage.test_coverages.length).toBe(8);
    expect(test_coverage.test_coverages[0].test_identifier).toBe('test added statement');
})

test('test coverage_task', async () => {
    if (existsSync('target/coverage_v1.json')) {
        unlinkSync('target/coverage_v1.json');
    }
    if (existsSync('target/coverage_v2.json')) {
        unlinkSync('target/coverage_v2.json');
    }
    await diff_jsjoules.coverage_task('test_resources/diff-jsjoules-toy-nodejs-project', 'test_resources/diff', 'target');
    const coverage_v1 = JSON.parse(readFileSync('target/coverage_v1.json', 'utf-8'));
    expect(coverage_v1.test_coverages.length).toBe(8);
    expect(coverage_v1.test_coverages[0].test_identifier).toBe('test added statement');
    const coverage_v2 = JSON.parse(readFileSync('target/coverage_v2.json', 'utf-8'));
    expect(coverage_v2.test_coverages.length).toBe(8);
    expect(coverage_v2.test_coverages[0].test_identifier).toBe('test added statement');
});

test('test instrumentation_task', async () => {
    const fs = require('fs');
    await diff_jsjoules.exec_command('cp test_resources/diff-jsjoules-toy-nodejs-project/src/app.test.js test_resources/diff-jsjoules-toy-nodejs-project/app.test.js.backup');
    await diff_jsjoules.exec_command('cp test_resources/diff-jsjoules-toy-nodejs-project-v2/src/app.test.js test_resources/diff-jsjoules-toy-nodejs-project-v2/app.test.js.backup');
    await diff_jsjoules.instrumentation_task('test_resources/diff-jsjoules-toy-nodejs-project', 'test_resources/diff-jsjoules-toy-nodejs-project-v2', 'test_resources/test_selection.json');
    const content_app_v1 = readFileSync('test_resources/diff-jsjoules-toy-nodejs-project/src/app.test.js', 'utf-8')
    const content_app_v2 = readFileSync('test_resources/diff-jsjoules-toy-nodejs-project-v2/src/app.test.js', 'utf-8')
    expect(content_app_v1.startsWith('const diff_jsjoules = require(')).toBeTruthy();
    expect(content_app_v2.startsWith('const diff_jsjoules = require(')).toBeTruthy();
    expect(content_app_v1.indexOf('diff_jsjoules.start')).toBeGreaterThanOrEqual(-1);
    expect(content_app_v2.indexOf('diff_jsjoules.start')).toBeGreaterThanOrEqual(-1);
    expect(content_app_v1.indexOf('diff_jsjoules.stop')).toBeGreaterThanOrEqual(-1);
    expect(content_app_v2.indexOf('diff_jsjoules.stop')).toBeGreaterThanOrEqual(-1);
    await diff_jsjoules.exec_command('cp test_resources/diff-jsjoules-toy-nodejs-project/app.test.js.backup test_resources/diff-jsjoules-toy-nodejs-project/src/app.test.js');
    await diff_jsjoules.exec_command('cp test_resources/diff-jsjoules-toy-nodejs-project-v2/app.test.js.backup test_resources/diff-jsjoules-toy-nodejs-project-v2/src/app.test.js');
})

test('test execution_task', async () => {
    await diff_jsjoules.execution_task('test_resources/diff-jsjoules-toy-nodejs-project', 'test_resources/diff-jsjoules-toy-nodejs-project-v2', 'test_resources/test_selection.json');
})