Promise.resolve().then(async () => {
  const module = await import("../crate/pkg");
  module.run();
});
