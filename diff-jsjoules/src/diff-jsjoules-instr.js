const process = require('process');
const dgram = require('dgram');
const { Buffer } = require('buffer');

async function send(message) {
    const client = dgram.createSocket('udp4');
    return new Promise((resolve, reject) => {
        client.send(Buffer.from(message), 2000, () => {
            console.log(`${message} has been sent!`)
            resolve(client.close());
        });
    });
}

async function start(test_identifier) {
    await send(`start ${process.pid} ${test_identifier}`);
}

async function stop(test_identifier) {
    await send(`stop ${test_identifier}`);
}

async function report(prefix_path) {
    await send(`report ${prefix_path}_${process.pid}.json`);
}

exports.start = start;
exports.stop = stop;
exports.report = report;