# insert_data

## Overview
cli tool. that insert data into template.
built by rust.


## Requirement
Rust,
Cargo

## Usage
insert_data -i input.txt -t template.txt -r 3
write output.txt

## options

### --input_data -i
input text file.
need space line
ex.
apple
banana

green
blue

### --template -t
template text file
place holder is like $1
ex.
<div>
<title>$1</title>
<p>$2</p>
</div>

### --row -r
you have to teach program "how many rows is chunk of data".
ex.
2
stands for 2 rows are chunk of data.(apple,banana)

### output file
text file
ex.

<div>
<title>apple</title>
<p>banana</p>
</div>
<div>
<title>green</title>
<p>blue</p>
</div>


## Author
masamichi

## Licence

MIT