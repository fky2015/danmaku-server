<!-- PROJECT LOGO -->
<br />
<div align="center">
<!--
  <a href="https://github.com/github_username/repo">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>
-->
  <h3 align="center">Danmaku Server</h3>
  
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

A [DPlayer](dplayer-url) Compatible Danmaku Server built with actix-web.
    
<!--
<br />
    
    <a href="https://github.com/github_username/repo"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/github_username/repo">View Demo</a>
    ·
    <a href="https://github.com/github_username/repo/issues">Report Bug</a>
    ·
    <a href="https://github.com/github_username/repo/issues">Request Feature</a>
-->    

</div>


<!-- ABOUT THE PROJECT -->
## About The Project

Danmaku Server aimed to be a B

a danmaku server based on [Actix-web][actix-web-url],
compatible with [DPlayer][dplayer-url].

### Feature

- [x] Authenticate with Oauth2. (Anonymous can only receive danmaku)
- [x] Multiple danmaku room
- [x] Configure from `.env`

### Built With

* [actix-web][actix-web-url]

## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

Nightly built Rust.

### Installation
 
```bash
git clone this-project
```
 
### Configuration

```Bash
# Required
# this config is required by OAuth2.
CLIENT_ID=OAuth2_id
# this config is required by OAuth2.
CLIENT_SECRET=secret
# this config is required by OAuth2.
REDIRECT_URL=https://danmaku.test.com
# this config is required by OAuth2.
TOKEN_URL=https://danmaku.test.com
# this config is required by OAuth2.
AUTH_URL=IP

# Optional, will be default if not provided
ADDRESS=0.0.0.0
PORT=80

```

### Start up

```bash
cargo run
```


<!-- USAGE EXAMPLES -->
## Usage

**TODO**

<!-- ROADMAP -->
## Roadmap

**TODO**

See the [open issues](https://github.com/github_username/repo/issues) for a list of proposed features (and known issues).



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

FKYnJYQ - [🐦twitter](https://twitter.com/FKYnJYQ) - loveress01@outlook.com

Project Link: [https://github.com/github_username/repo](https://github.com/fky2015/danmaku-server)


<!-- ACKNOWLEDGEMENTS
## Acknowledgements

* []()
* []()
* []()

-->



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/fky2015/danmaku-server.svg?style=flat-square
[contributors-url]: https://github.com/fky2015/danmaku-server/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/fky2015/danmaku-server.svg?style=flat-square
[forks-url]: https://github.com/fky2015/danmaku-server/network/members
[stars-shield]: https://img.shields.io/github/stars/fky2015/danmaku-server.svg?style=flat-square
[stars-url]: https://github.com/fky2015/danmaku-server/stargazers
[issues-shield]: https://img.shields.io/github/issues/fky2015/danmaku-server.svg?style=flat-square
[issues-url]: https://github.com/fky2015/danmaku-server/issues
[license-shield]: https://img.shields.io/github/license/fky2015/danmaku-server.svg?style=flat-square
[license-url]: https://github.com/fky2015/danmaku-server/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=flat-square&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/othneildrew
[product-screenshot]: images/screenshot.png
[actix-web-url]: https://github.com/actix/actix-web
[dplayer-url]: https://github.com/MoePlayer/DPlayer


