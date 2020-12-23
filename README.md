<!--# BattleShip
A simple console based implementation of the classic BattleShip game in Rust,
with an AI. The AI uses a stack to hold adjacent coordinates to any point
that was successfully attacked. AI guesses are pulled from the stack as long
as it is not empty. Otherwise, a point is randomly chosen. The board will be
re-printed every round to update the graphic. Here is an example:

Round 8:
Please input your guess (x and y, separated by a space).
2 5
You guessed: (5,2)
a ship has been sunk!
HIT!
Computer attacked (3,4), and missed!
       A    B    C    D    E    F    G    H    I    J 
     __________________________________________________
1   |  .    .    .    .    .    .    .    .    .    .  |
2   |  .    .    .    .    .    .    .    o    .    .  |
3   |  .    .    .    .    .    .    .    .    .    .  |
4   |  .    .    o    .    o    .    .    .    .    .  |
5   |  .    X    X    X    X    .    .    .    .    .  |
6   |  .    .    .    .    .    .    .    .    .    .  |
7   |  .    .    .    .    .    .    .    .    .    .  |
8   |  .    o    .    .    .    .    .    .    .    .  |
9   |  .    .    .    .    .    .    .    .    .    .  |
10  |  .    .    .    .    .    .    .    .    .    .  |
     __________________________________________________


       A    B    C    D    E    F    G    H    I    J 
     __________________________________________________
1   |  ~    ~    ~    ~    ~    ~    ~    ~    ~    ~  |
2   |  ~    ~    ~    ~    ~    ~    ~    ~    ~    ~  |
3   |  ~    ~    ~    ~    ~    ~    ~    ~    ~    ~  |
4   |  ~    S    ✔    ✔    ~    ~    ~    ~    ~   ~  |
5   |  ~    ~    ~    D    D    ~    ~    ~    ~    ~  |
6   |  ~    ~    ~    C    C    C    C    C    ~    B  |
7   |  ~    ~    ~    ~    ~    ~    ~    ~    ~    B  |
8   |  ~    ~    ~    ~    ~    ~    ~    ~    ~    B  |
9   |  ~    ~    ~    U    U    U    ~    ~    ~    B  |
10  |  ~    ~    ~    ~    ~    ~    ~    ~    ~    ~  |
     __________________________________________________

-->
# BattleShip

<!--
*** Thanks for checking out the BattleShip. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Thanks again!
-->



<!-- PROJECT SHIELDS 
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]
-->


<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/GalacticWafer/BattleShip">
    <img src="https://galacticwafer.github.io/images/radar.svg" alt="Logo" width="160" height="160">
  </a>

  <h3 align="center">BattleShip</h3>

  <p align="center">
    A simple console based implementation of the classic BattleShip game in Rust,
with an AI.
    <br />
    <a href="https://github.com/GalacticWafer/BattleShip"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/GalacticWafer/BattleShip">View Demo</a>
    ·
    <a href="https://github.com/GalacticWafer/BattleShip/issues">Report Bug</a>
    ·
    <a href="https://github.com/GalacticWafer/BattleShip/issues">Request Feature</a>
  </p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://example.com)

Rust is a challenging, but great language. BattleShip is a great choice for a game which can be implemented with Rust to learn the basics of borrowing and lifetimes.

The player and the AI will have their ships randomly placed on the board, and the player goes first. Here is a picture of what the gameplay might look like:
![Gameplay](https://raw.githubusercontent.com/GalacticWafer/BattleShip/master/images/gameplay.png)

<!-- GETTING STARTED -->
## Getting Started

<!-- This is an example of how you may give instructions on setting up your project locally. -->
To get a local copy up and running follow these simple example steps.

### Prerequisites

1. cargo
First make sure you have the rust toolchain installed. See [here](https://example.com) for more details on getting set up with rust.

2. Open a terminal window and clone this repository.
```sh
git clone https://github.com/GalacticWafer/BattleShip.git
```

<!-- USAGE EXAMPLES -->
## Usage

Use this space to show useful examples of how a project can be used. Additional screenshots, code examples and demos work well in this space. You may also link to more resources.

_For more examples, please refer to the [Documentation](https://example.com)_



<!-- ROADMAP -->
## Roadmap

See the [open issues](https://github.com/GalacticWafer/BattleShip/issues) for a list of proposed features (and known issues).



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.



<!-- CONTACT -->
## Contact

Project Link: [https://github.com/GalacticWafer/BattleShip](https://github.com/GalacticWafer/BattleShip)




<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links 
[contributors-shield]: https://img.shields.io/github/contributors/othneildrew/Best-README-Template.svg?style=for-the-badge
[contributors-url]: https://github.com/GalacticWafer/BattleShip/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/othneildrew/Best-README-Template.svg?style=for-the-badge
[forks-url]: https://github.com/GalacticWafer/BattleShip/network/members
[stars-shield]: https://img.shields.io/github/stars/othneildrew/Best-README-Template.svg?style=for-the-badge
[stars-url]: https://github.com/GalacticWafer/BattleShip/stargazers
[issues-shield]: https://img.shields.io/github/issues/othneildrew/Best-README-Template.svg?style=for-the-badge
[issues-url]: https://github.com/GalacticWafer/BattleShip/issues
[license-shield]: https://img.shields.io/github/license/othneildrew/Best-README-Template.svg?style=for-the-badge
[license-url]: https://github.com/GalacticWafer/BattleShip/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/othneildrew
[product-screenshot]: images/screenshot.png
-->
