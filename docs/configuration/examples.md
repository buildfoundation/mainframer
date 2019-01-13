# Configuration Examples

## `ignore.yml`

### General

```yaml
push:
  # Build results are going to be invalidated on the remote machine.
  # Replace "out" with the directory where build results are placed.
  - "out"
pull:
  # The source code on the local machine is more important than remote one. 
  # Replace "src" with the directory where the source code is placed.
  - "src"
```

### Tools

#### Git

```yaml
push:
  # VCS directories are usually heavy and not required for the build.
  - "/.git"
```

#### IntelliJ IDEA

Includes Android Studio, PyCharm, WebStorm, GoLand and other tools built
on the IntelliJ Platform.

```yaml
push:
  # IDE-specific directories are not required for the build.
  - "/.idea"
  - "*.iml"
```

#### Android Studio

```yaml
push:
  # Profiling files are not required for the build.
  - "/captures"
both:
  # Contains machine-specific files.
  - "/local.properties"
```

### Build Systems

#### Buck

```yaml
push:
  # Build results are going to be invalidated on the remote machine.
  - "buck-out"
both:
  # Contains machine-specific files.
  - "/.buckd"
```

#### Cargo

```yaml
push:
  # Build results are going to be invalidated on the remote machine.
  - "target"
both:
  # Remove Cargo.lock if creating an executable, leave it for libraries.
  # More information: http://doc.crates.io/guide.html#cargotoml-vs-cargolock
  - "/Cargo.lock"
```

#### Go

```yaml
push:
  # Build results are going to be invalidated on the remote machine.
  - "bin"
```

#### Gradle

```yaml
push:
  # Build results are going to be invalidated on the remote machine.
  - "build"
both:
  # Contains machine-specific files.
  - ".gradle"
```

#### Maven

```yaml
push:
  # Build results are going to be invalidated on the remote machine.
  - "target"
```
