const base = std.pluginDir();
const inputPath = base + "/data/input.txt";
const outputPath = base + "/data/output.txt";
const body = std.readTextFile(inputPath);
std.writeTextFile(outputPath, body.trim() + "\nprocessed by scoped-fs\n");
std.emit({
  plugin: "scoped-fs",
  input: body.trim(),
  output: std.readTextFile(outputPath).trim()
});
