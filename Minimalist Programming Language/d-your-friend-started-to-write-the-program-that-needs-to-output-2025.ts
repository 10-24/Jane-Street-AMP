  const NUM = 2025;
  const findCheapestPath = cachedFindCheapestPath();
  const { route, cost } = findCheapestPath(NUM, 0);
  console.log(`Route: '${route}'`);
  console.log(`Cost: ${cost}`);
  console.log(`Route Length: ${route.length}`);
  
  function cachedFindCheapestPath() {
    const cache = new Map<number, Path>();
  
    function findCheapestPathWrapper(target: number, saltCounter: number): Path {
      const cachedPath = cache.get(target);
      if (cachedPath !== undefined) return cachedPath.clone();
      const path = findCheapestPathBase(target, saltCounter);
      cache.set(target, path);
      return path;
    }
  
    return findCheapestPathWrapper;
  
    function findCheapestPathBase(target: number, saltCounter: number): Path {
      // console.log(saltCounter)
      if (target === 1) return new Path("a", 1);
      if (target === 2) return new Path("f(a)", 3);
      if (target === 3) return new Path("g(a)", 4);
  
      const pathUsingFunctions = findCheapestUsingFunctions(target);
      // return pathUsingFunctions;
      const pathUsingFactors = findCheapestUsingFactors(target);
      return pathUsingFunctions.min(pathUsingFactors);
  
      function findCheapestUsingFactors(target: number) {
        const smallestFactorPair = calculateSmallestFactorPair(target);
        // is Prime
        if (smallestFactorPair === undefined) {
          const up = findCheapestPathWrapper(target + 1, 1); // Shout out Mersenne primes
          const down = findCheapestPathWrapper(target - 1, -1);
          const cheaperPath = up.min(down);
          return cheaperPath.push(new Path("-a", 2)).wrapInParenthesis();
        }
  
        const [factor1, factor2] = smallestFactorPair;
  
        const factor1Path = findCheapestPathWrapper(factor1, 0);
        const factor2Path = findCheapestPathWrapper(factor2, 0);
  
        return factor1Path.pushWithMathOperator("*", factor2Path);
      }
  
      function findCheapestUsingFunctions(target: number) {
        if (target % 2 === 0)
          return new Path("f", 3).pushInParenthesis(
            findCheapestPathWrapper(target / 2, 0)
          );
  
        return new Path("g", 3).pushInParenthesis(
          findCheapestPathWrapper((target - 1) / 2, 0)
        );
      }
    }
    function calculateSmallestFactorPair(num: number) {
      let factorPair:[number,number] | undefined = undefined;
      const factors = new Set();
      for (let i = 2; i <= num / 2; i++) {
        const q = num / i;
  
        if (Math.floor(q) !== q) continue;
  
        if (factors.has(q)) break;
  
        factors.add(i);
        factorPair = [i, q];
      }
  
      return factorPair;
    }
  }

  class Path {

    constructor(public route: string, public cost: number) {}
    push(other: Path) {
      this.route += other.route;
      this.cost += other.cost;
      return this;
    }
    pushInParenthesis(other: Path) {
      this.route += other.wrapInParenthesis().route;
  
      this.cost += other.cost;
      return this;
    }
    pushWithMathOperator(operator: string, other: Path) {
      this.route = `${this.route}${operator}${other.route}`;
      this.cost += operator.length + other.cost;
      return this;
    }
    min(other: Path) {
      if (this.cost === other.cost) {
        if (this.route.length < other.route.length) return this;
        return other;
      }
  
      if (this.cost < other.cost) return this;
      return other;
    }
    wrapInParenthesis() {
      this.route = `(${this.route})`;
      return this;
    }
    clone() {
      return new Path(this.route, this.cost);
    }
  }
