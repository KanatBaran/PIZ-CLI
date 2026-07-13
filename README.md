# PIZ CLI

PIZ CLI is an open-source command-line interface (CLI) application developed to increase file size in a controlled manner. Unlike traditional compression tools, PIZ CLI expands files to a desired size using the digits of π (Pi) or random data. In this respect, it aims to become one of the world's first CLI tools dedicated to file expansion.

## WHY PIZ?

The open-source world has written millions of lines of code. It has developed operating systems, created programming languages, and shaped the internet.

But what if that same community could also make helping others visible?

PIZ is more than a CLI that expands files. It is an idea: directing the attention of the open-source community toward social responsibility.

When you write a line of code for PIZ, you don't just add a new feature. You also help someone, somewhere in the world, discover a charity they've never heard of before.

Perhaps the feature you write will attract the attention of thousands of people. Perhaps just one of them will support a child's education, a disaster area, or an organization fighting hunger.

We can't measure it.

But we know that many things that have changed the world began with the curiosity of a single person.

If the code you write can affect not only computers but also people...

Then why not?

**Welcome to PIZ. Write code. Spark curiosity. Do good.**

## HOW CAN YOU HELP?

PIZ exists to connect open source with social good, and everyone can contribute in their own way.

- **Support a cause.**  
  If you're in a position to do so, visit **piz.world** and consider supporting one of the charities or social initiatives featured there. Even a small contribution can make a difference.

- **Contribute to PIZ.**  
  If you're a developer, you can help improve PIZ by fixing bugs, implementing new features, improving documentation, or suggesting new ideas.

- **Spread the word.**  
  If you have an audience on social media, consider sharing **piz.world** or the PIZ project. A single post may help someone discover a cause worth supporting.

- **Represent your country.**  
  If you live in another country, you can help localize **piz.world** into your language and contribute trusted local charities and social organizations. Together, we can make PIZ meaningful for people all around the world.

Every contribution matters. Whether you write code, translate content, share the project, or support a charity, you're helping build a global community where technology inspires people to do good.

Note: PIZ is currently under development. Initial versions are experimental, and the project will be expanded over time with new features and components focused on social benefit.

## Commands

### General

```bash
piz --help
piz --version
```


### Expanding File Size

You can increase the size of a file by a specific amount or make it reach a specific target size.

#### Adding a specific size

```bash
piz expand <file> --add <size>
```

**Example:**

```bash
piz expand photo.jpg --add 1GB
```

Appends **1 GB** of data to the end of the `photo.jpg` file.


#### Specifying target file size

```bash
piz expand <file> --size <size>
```

**Example:**

```bash
piz expand photo.jpg --size 2GB
```

Makes the final size of the `photo.jpg` file **2 GB**.


#### Specifying the fill method

If the `--fill` parameter is not specified, PIZ defaults to using **the digits of the number π (Pi)**.

**Examples:**

```bash
piz expand photo.jpg --add 1GB --fill random
```

```bash
piz expand photo.jpg --size 2GB --fill pi
```

Supported fill methods:

* `pi` *(default)*
* `random`


#### Specifying the output file

By default, PIZ does not modify the original file and automatically creates a new output file.

If desired, the output file can be specified with the `--output` parameter.

**Example:**

```bash
piz expand photo.jpg --add 1GB --output large-photo.jpg
```


## Rules

* **Only one** of the `--add` or `--size` parameters must be used.
* The `--add` and `--size` parameters cannot be used at the same time.
* **At least one** of the `--add` or `--size` parameters must be specified.
* The value provided with `--size` cannot be smaller than the current file size.
* If `--fill` is not specified, the default value is assumed to be `pi`.
* Supported fill methods:

  * `pi`
  * `random`
* If `--output` is not specified, PIZ automatically creates a new output file and does not modify the original file.


## Usage Examples

```bash
# Adds 1 GB to the file size (uses pi digits by default)
piz expand photo.jpg --add 1GB
```

```bash
# Adds 500 MB of random data to the file size
piz expand photo.jpg --add 500MB --fill random
```

```bash
# Makes the final size of the file 2 GB
piz expand photo.jpg --size 2GB
```

```bash
# Saves the output to a different file
piz expand photo.jpg --size 2GB --output photo-2gb.jpg
```