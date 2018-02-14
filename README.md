# The GAI Library
## A very cheerful library for Artificial Intelligence in games

Hello, my name is Maciej Gorywoda, and this is my attempt at creating a library for Artificial Intelligence in games, and at the same time to learn Rust. 

Right now I work as a Scala programmer at [Wire](https://wire.com/en/) in Berlin, but twelve years ago I graduated from Warsaw University of Technology with MSc and specialization in Artificial Intelligence, and then I worked for two years for a small game programming company, coding simple aircraft shooters. Recently I started to think about getting back tothe subject (more like dipping a toe, actually) and writing a small AI library for games in my free time. But in the meantime [C++ turned into a monster](https://yosefk.com/c++fqa/) and I certainly didn’t want to use it again. For some time I was thinking about a mix of C and Lua. They bind together very well. I like the minimalism of C, and Lua is a great little scripting language for more high-level concepts, while still being quite fast (especially LuaJIT). Then I learned about Rust and all the puzzle pieces came together. Rust is similar enough to Scala that I can think about how would I code things in Scala and most of the time I can apply it to Rust. Rust is also popular enough and mature enough that I don’t think it will go away in a few years – and it seems to pretty well fit for writing computer games in it. Furthermore, learning while actually writing something useful is the best way for me. And there is this webpage, [http://arewegameyet.com/](http://arewegameyet.com/), which keeps track of games and libraries written in Rust. And you know what? There’s a big hole in the area of AI libraries for games. We either use big game engines, or write AI ourselves, inventing wheel again and again. So I thought I could do something about it.

This is my first project in Rust. I plan to bring in all the programming experience I have, but still, well, I din't write much in Rust till now, and it will show in the code. Fortunately there are ways to remedy that. I plan to create tickets in GitHub for the things I want to do, use TDD, write a lot of integration tests, document the stuff I write along the way with rustdoc, perform stress tests before deciding on optimizations, and maybe even write short blog notes about what I’m doing and what I’m using. In a way, you may look at this project as an exercise in the Agile development.  I’m sure it will be a slow process and I will make mistakes which will force me to get back to rewrite parts of the code. It will be painful. There will be a lot of sweat and tears. I only hope that at the end I will have a nice crate for other game programmers to use, together with a description of the creative process, and my head will be full of Rust.  

### The current state of affairs

The initial design document is ready:
* [The Initial Design main page](https://github.com/makingthematrix/gailibrary/wiki/Initial-Design)

It's quite long, still contains some errors (I'm not a native English speaker) and in some place it feels like a stream of conscioussness. I will be coming back to it over time and edit it.

* [My current backlog and the sprint table](https://github.com/makingthematrix/gailibrary/projects/1)

It's based on the first half of the initial design doc - up to the use case.Now I will slowly work through the tickets. I hope I will be able to finish it in 2018 (I'm writing these words on the New Year's Eve :) ). Still, if you have any thoughts, comments, wishes, questions, whatever, feel free to contact me. You can open an issue or you can reach me on Twitter ([@makingthematrix](https://twitter.com/makingthematrix)) or Wire (@maciek).
