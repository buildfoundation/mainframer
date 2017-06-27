## Sample Maven project

Most interesting for you here is not the source code, but configs in [`.mainframer`](.mainframer) folder.

This is a very common setup for basically any Maven project (including multimodule), but of course you might need to tune some ignore configs for your project.

### How to build

```bash
$ ./mainframer ./mvnw package
```

Or any other Maven goal you want, btw it integrates with IntelliJ pretty easily! TODO add link to docs.

### Requirements

JDK 6+ installed on remote machine.

BTW, Maven users, we strongly recommend you use [Maven Wrapper](https://github.com/takari/maven-wrapper) so remote machine will install required version of `mvn` transparently. 

### Recipes

* [IntelliJ Run Config Recipe](../../recipes/INTELLIJ_RUN_CONFIG.md)
