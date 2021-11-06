# cs128h-project
Project for CS128 Honors

## Group
| Name | NetID |
| ------------- | ------------- |
| Andrew | alester3 |
| Arul | arulhv2 |
| Devak | devakn2 |
| Jonathan | jsneh2 |

## Project Introduction
The Project focuses on creating a web application that runs a Map Reduce on any uploaded dataset along with other dataset analysis tools (To be decided) using a rust backend. The application will be tested using pre-existing datasets found from online (Will be sourced as we use them). We chose to work on this project because we all enjoyed the last MP, and we think that working more on running parallel data analysis algorithms will improve our understanding of Rust and concurrency.

Some goals we have for the project are a simple interface with not much code, interesting data analysis features, and benchmarking for our concurrent algorithms. We believe that most of the work in the project should be spent on implementing the parallel algorithms, so the user interface will be rather simple to make up for that. Our data analysis features could involve graph or tree algorithms, image processing, or other data visualization.

## System Overview
Major technical checkpoints will include:
1) Choosing a web framework for our Rust backend
2) Developing a front end of some sort using html and css styling
3) Developing a map reduce algorithm using rust to process data in a time efficient manner
4) Testing our algorithm using online datasets and publishing results
5) Determing what other algorithms we could implement
6) Hook up the algorithms to the web backend. In doing so, find ways for the user to upload data for the algorithms to run, whether that be through images or even video processing.

## Possible Challenges

One possible challenge will be identifying a good backend library to use with Rust and hooking our data analysis code into it. There are many of them out there, so we will need to make a careful decision at the start so that we are headed in the right direction. Another challenge will be designing the process for the user to upload datasets to our backend for use by the MapReduce and/or other parallel data processing algorithms. If a data set is truly large, it might not be possible to upload it directly and we would have to resort to some form of data streaming.
