module.exports = function(eleventyConfig) {
  eleventyConfig.addPassthroughCopy({ "assets": "assets" });
  eleventyConfig.addPassthroughCopy("usage");
  eleventyConfig.addPassthroughCopy("*.png");
  eleventyConfig.addPassthroughCopy("*.gif");

  return {
    dir: {
      input: ".",
      output: "_site"
    },
    markdownTemplateEngine: "njk",
    htmlTemplateEngine: "njk",
    dataTemplateEngine: "njk"
  };
};
