import init, { greet } from './pkg/snake_rs.js'
async function run() {
    await init();
    greet('fff');
}
run();
