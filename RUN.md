## User Guide

* Clone the repo ```git clone https://github.com/Chaoqi-LIU/Huffman_Coding```
* Move to the dir ```cd huffman```
* To see all operations provided ```cargo run -- help```
  <pre>
    -- help                                                 print this help message
    -- encode input_file -o output_file huffman_tree        encode the content in 'input_file'[.txt], write encoded content to 'output_file'[.dat], and save the huffman tree to 'huffman_tree'[.crab]
    -- encode input_file huffman_tree -o output_file        encode the content in 'input_file'[.txt] with 'huffman_tree'[.crab] provided, write encoded content to 'output_file'[.dat]
    -- decode input_file huffman_tree -o output_file        decode the content in 'input_file'[.txt] with 'huffman_tree'[.crab] provided, write decoded content to 'output_file'[.txt]
    -- compress input_file -o output_file huffman_tree      compress the image in 'input_file'[.ppm], write compressed image to 'output_file'[.dat] and save the huffman tree to 'huffman_tree'[.crab]
    -- compress input_file huffman_tree -o output_file      compress the image in 'input_file'[.ppm] with 'huffman_tree'[.crab] provided, write compressed image to 'output_file'[.dat]
    -- depress intput_file huffman_tree -o output_file      depress the image in 'input_file'[.dat] with 'huffman_tree'[.crab] provided, write depressed image to 'output_file'[.ppm]
    -- print huffman_tree                                   print the 'huffman_tree'[.crab]
  </pre>
