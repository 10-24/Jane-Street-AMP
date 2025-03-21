import { init } from 'z3-solver';

find_solution();
testSolution();

async function find_solution() {
	const { Context } = await init();
	const z3 = Context('main');
	const { Int, Real } = z3;

	const solver = new z3.Optimize();

	const k = Int.const('k');
	const n = Int.const('n');

	const k_squared = z3.ToReal(k).pow(2);
	const n_squared = z3.ToReal(n).pow(2);

	const three = Real.val(3);
	const three_to_the_n_squared = three.pow(n_squared); // 3^(n^2)
	const left_eq = k_squared.mul(three_to_the_n_squared); // k^2 * 3^(n^2)
	const right_eq = Real.val(2025);

	const costFn = k.mul(2).add(n.mul(2)).add(Int.val(2));

	solver.minimize(costFn);
	solver.add(right_eq.eq(left_eq));
	solver.add(n.ge(0));
	solver.add(k.ge(0));
	solver.add(k.le(12));
	solver.add(n.le(12));

	if ((await solver.check()) === 'sat') {
		const model = solver.model();
		console.log('Value of k:', model.eval(k).toString());
		console.log('Value of n:', model.eval(n).toString());
		console.log('Min Cost:', model.eval(costFn).toString());
	} else {
		console.log('No solution found.');
	}
}

function testSolution() {
	const g = (x: number) => 5 * x;
	const f = (x: number) => Math.pow(x, 2);
	const output = g(g(f(f(3))));
	console.log(output);
}
