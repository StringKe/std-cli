const input = std.args();
std.emit({
  plugin: "hello-js",
  greeting: "hello from std-cli",
  input
});
