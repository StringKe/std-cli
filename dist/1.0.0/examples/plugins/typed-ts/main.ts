type Input = {
  name?: string;
};

type Output = {
  plugin: string;
  greeting: string;
};

const input: Input = std.args() as Input;
const name: string = input.name || "std-cli";
const output: Output = {
  plugin: "typed-ts",
  greeting: "hello " + name
};

std.emit(output);
