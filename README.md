# eksplein ![status](https://img.shields.io/badge/Made_with-Rust-B57342) [![Build Status](https://drone.tommywalkie.com/api/badges/eksplein/eksplein/status.svg)](https://drone.tommywalkie.com/eksplein/eksplein) <img align="right" width="120" height="120" src="https://avatars3.githubusercontent.com/u/68416933?s=200&v=4">

> explain — **_verb_** [ɪkˈspleɪn, ɛkˈspleɪn]<br>
> <sup>_Make (an idea or situation) clear to someone by describing it in more detail or revealing relevant facts. ([source](https://www.lexico.com/definition/explain))_</sup>

This is the core repository for the Eksplein project, including the source code for the official Actix-based server. Eksplein is supposed to be sort of a modern glossary-like blog in which any given problem meets _high-level_ explanations.

<sup><em><u>Side note</u> : Logo is still early concept.</em></sup>

## Motivation

Primarily targeting computer scientists and media enthusiasts, Eksplein is expected to come with a set of innovating features, addressing various issues in existing glossaries and educational websites.

- Lack of interoperable tutorials (_cf._ languages, frameworks, OSes, versions)
- Bloated explanations
- Spoilers, spoilers everywhere
- _You read a lot. We like that.™_

## Stack

#### Backend

- **Rust** ![Rust](https://img.shields.io/docker/v/_/rust/1?label=Rust&color=B57342&logo=rust) — High-performance memory-safe language
- **Actix** ![Actix](https://img.shields.io/badge/crates.io-v2.0.0-FC8D62?label=Actix&color=156060&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAB4AAAAeCAYAAAA7MK6iAAAERUlEQVRIS8WWaaimcxjGfxczZowRMWnGFmb4YrLLEtmzL9mStTAfhCLF2LJMGNsX5APygawzDRKSrUi2kiVCliEMGctYRgZz6Trdz9tz3vOc857zyV2n9zz9//97ve7rvsX/JJqIXdvrAlsDnwJHAw8Bx0habHuSpH/Gq2+gYdu5szFwPnA/8ABwPbAQuKh+dwPeArYBtpP07iAHxjRsey3gaeAOYAnwE7Bhh9LfgT+AS4ETgSOA9SX9OJoDoxq2fQ3wIrAYmAF8BjwPPAa8LmmF7TWBmcChwKnAPsD7Ff2zkh4dt2HbmwOzgDOAc4CvgYslpZ5jiu2Tqu65dzhwJnCapD/7H46I2HaMfgVMAp5L2iStGmS0Obe9Eli7vv9OyoMRSQFkT4YZrvTOBk4B3gR2l+QJGJ0OrKiyzAGiP+8PlJSyjTRsO15uAXxYp7n8wgSMJrJXgO8kHWQ76D+53u+c1Eta0OjrRWz7LmBey9Cekl4bZNh2ogyongACtuXA3sBLBbxGxUxJ3w8zbHtKITdAauQT4GBgmaS/WjVM7dcrpRcWgPqxkvQGjDGe7ggPXBW0S3omuoYe2E5N7wXSt5FLgIDk9vrO/2GlOJi/sSRGA8j0/5DYDsjCAbMkrW4bntrQX91NS4UQugggD9+oCPYFLgCmtTxZIum4tme2X6703wxcLWllE/GrwB5NBgZEdKekc1vRpP2+qbe/SkoZhontXatL0lbL2hGvAXwBJNIAKpHsBNxXQ6GtaEp/X9tOq+wHXCfpii7HbacEtwELJC1vIv64CD6H05OKVkTbFg3mbtI8VVJq1hPbabu96mxE39egydutgKXhhsZwfmN8mqRNO1KVHg8Hh1wWSsowaICzUXq3FEbxCLF9GPAUcANwpaR/G8P3AGeNFlEhc3KVYceaWLl/LHALsA4wX9KNoxgOhlLnLas9V7cJJAoyX/eXlP7r8jyRZ0oFUP0yR1LOut5lSIRc5koKP/T6OO0UEt8kbSWpobouJeHgPG6TRuqaDWSoR9tSW0v4ewcgqF/aM1ypjKIvi2V2kfROl/d1N2lOeRpZKSnp7nI07ZOZnYxmvA6Br53qrC2Zv+cBH8XDgGAUZWGv8G7Ts6skjWA028cDi4DggnYw/WMxm0ZSEe8XScoa0ym2QyJZiRqZLenz5sN2Jl1qnohDl9u3R2y/4YBnLnAtcAjwZJDbtT0WGz1SRBPCuVxSkBtuzvZydy0T4f8ZktJyPencuWxnrwpJnAAkinn9g9z2AcCDxWwpUbbOb4GUZzPgZ+Ao4AdJ4YhhMtayF+5+vKZSwPF2LXrvVT9mzHVtnDF8NnCrpK7zIQfGMpy5mwX+9Io81JktYyyJM8FFWmdy15LXPB7PQr8B8BtwU02Y+cVgRwJhpCzzlwGhzoezgfRzeZenAw33P7KdaD4og+HvrD6/SMr8HrdM2PC4NQ+4+B+X/5Au8e+D6wAAAABJRU5ErkJggg==) — Rust-based small, pragmatic, and fast web framework
- **Redis** ![Redis](https://img.shields.io/docker/v/_/redis/6?logo=redis&label=Redis&color=D92B21&logoColor=ffffff) — High-performance in-memory key–value database
- **Juniper** ![Juniper](https://img.shields.io/crates/v/juniper?label=Juniper&color=E10198&logo=graphql) — GraphQL server library for Rust

#### Frontend 

Repository → [`eksplein/website`](https://github.com/eksplein/website)

- **Sapper** ![Sapper](https://img.shields.io/github/package-json/dependency-version/eksplein/website/dev/sapper?filename=package.json&label=Sapper&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAB4AAAAeCAYAAAA7MK6iAAAEGUlEQVRIS8VXX2hTVxz+zk3aPtlWTBTsnNaHoSg2A2thtuiqhc0X63wQhGl8UNE0iQF9c9jp2xTSJI1iQdox2NAHrQ+jD07XbTJsq5iKYn1wrcoGazKI7VOT5pzxu/Wk597ce5M5wd9L23v+fOf3ne/7/U4Z3lOw/4NbH720xu0SqzUmttM+XLDh+cLceDYSyZbb962AvdGkDy6EGeC3ARjMFbRINnJsyu4A/xl4eSzZDYYz5TISQmTBWCQdCgxYza0Y2JNIbNe41g+GNepGq5YswWdrG/VPv//5F55kMgYcARyyAi8LXB+N1le5qs4wsBPqjrXV1Yh17MCuN6By7HE6g9BPd4oHoMzzPN9ovndH4GWJi50a5/2MsXoV9IhvE05uaUZdTY0l46/n5rDjh2t4NTurjwshvk2Huwx6sAX2xpI9jCFspjXW0Y6tDQ0GwJlcDgRGtMu4+nRCz/wNcDYd7lqqLrIE9sR7T2hgUXXiqZZmPUtzDP0xidCt2yDw+M527Fu/Tp9CB/mo70pxeq6gNaoqLwEmb1ZphYeS3g0ej77hRq/HgEk0EiAJSgbNHdzbCbp/ihWJi8UxzvinmWBwWH4oAVbtQhvc939Zcpd9qUc4PzKqZ2kVlDUxtHngu8oz9saTwwzYRitIROfaWouLySrBW4uKtUS1+CiA1+lQwCDQkoy98WSWAXW0/sYXnfikYaW+FVG65/qgYVsaO+xrQl3NArVkpQsjYyVMVKTq5fHeKYCtNgOrKrXzsBTVnus3DYVEFPBxOhJIOapapZpoJrplXE6NY2Yup3+z87AEp/uVGrCqXiVUq1aizB4cOlBUqdWd0uZ9qXHUVtcYDvnVb3dBItRD4OZ0ONDpnHE06WMuPLSziLpY9bCTJgTwSzoU0FunrZ28sd4BxthBdRJlfqplS9HLVBwoS9XDNIfEKP1uFKN4MR3qMjQXR1VXahdSN+lBLTIq1WUzpgav0kzF45t7o7g28czyDJTlyZZmHPU1GcapqrV/f3XRVgJfT4cD3bZU6z1XaD/LCX8Hj+u/vpyZhf/HIYNFPl/biLNtrfiwdrEx0FxzkaHikS9oPvNrxEA11elqF580A9PfF0bHcH5kTFe4VR+W6qY5anCISCbU1WOmzPGO1W4jM6cqZfYwCYkahuy/EkQIxNLhgOEBUZGqqb/e2b/P1seU5elf74KqmhpEL2fM/0/wuLHGOtlJb4sunpL1mlpdoqMd9FMNAiNQc4eiLPM8113uiWv5EPDGk34G9KtABLz1g4WGMfR8soRWQLzgTPjVnutkR/unTzxJb6Qembmjpy3sUq4GOD72dJVrvAcMu602onaX565up4e73QHKPm9pIT1x3W63T+Pawr8qGh+en3dPvQ2grarLUfSuxv8FrzLqLgDlI1MAAAAASUVORK5CYII=&color=159497) — Svelte-based server side rendering framework 
- **Svelte** ![Svelte](https://img.shields.io/github/package-json/dependency-version/eksplein/website/dev/svelte?filename=package.json&label=Svelte&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAB4AAAAeCAYAAAA7MK6iAAADY0lEQVRIS8WXMUxTURSGv1vUCSNuOAEJzpbNGAplM3EQEwyjJaySlk0Tk5ZJN0qYFRiNDLDhRKHEuFlmSSiTjG3CBMg15933+u7te68PcfBOcHt7/vuf+//nnCr+01L/gqsfM8xthlDkvTiaGn0cqhqttLg3AtZPyNJHEUUhFkCzxQWL6jvNpAv8NbCeoAKU0xgBLTSLqs563NlrA+tx8mRYA4adQIPDMP7cbDX24Kjh4mjm4sBTgXWeAX5TRlFyIvYPwJs1yE27QD8b8GHOvkCLDCPd794TWOeYRnksB5zoMyWYK4OAx62zFsyPwan/xJoNVXf1kAisJ6gCxUhaheWYEXFnnbVBwAaHwr2dDXjf0V5L7XPf/kossM5RQrHsBJ+rQCFGUwfbBkCA367D01fma/L/MwvrnBFb5RFgz5t3+NFJ72jWvOXDrMvy9MQANmrhvpxdqUH/PbM3aYW/Ykod0DkcBbbtIm/4+Tj6lpsrsFYxrOLW0wJIhmYtA6QyzlFDMenFExEtWBk/OjQsuy0TD2/vttW+K9A4xkLD5ErSljV38Dxa7BJVNg8zxTAjcqH1pWgmrqNqnaOJwsjTBrZVmuThQFTFKTcrl4ypbziVJcrYTvVC1TAK1peqYfOylOzhAHx2JGQeU73igEMreeJqhiqNe0vx8GbVXMS+5Oqi2Zel2VZ1nBIXBZbOc8uzk1ndFrHBbQ/30oRmT9X91ul/P47xOgq/CvinhI3YQy4RpFLSbntYzlR3Q7/bYtScqLrbXHqrOt0m5oSo+/WyW2TcVPdm7DV4O83yvp/K8HUj/grCUsqoiM1eUtXms7atltS+18c7y2Hs99zdzqd72vz5qwnvXrgWGZ82LB+47ZlokWlzTrZ7GnGBTZ0+jgDLhhQGKZNJHg7ULWfsZaYQX94JjGVbT3iDmqlcdrcJmN8diHpYhCSlNOi/YfwVtd81QFxL1TLWfGwk+1hYrpZgJzJWtdEUVJ2tJH0mtUUpb4a1WEiYjz5yY0gJFdBoh1ohQyVtxE0aBAr+yBOCyQXENrIOtqJp1Zx4LK2em8RW9pNHnxwCLqLwu3qvMETs0vN0L2BPaOaXQhWFP792hdNscEGl1+B+7TeOO+iNuJdkyfj19ooalzRvAhjET52r01J208//ACVGQy4B+QKGAAAAAElFTkSuQmCC&color=FF3E00) — Boilerplate-free JavaScript frontend framework
- **TypeScript** ![TypeScript](https://img.shields.io/github/package-json/dependency-version/eksplein/website/dev/typescript?filename=package.json&label=TypeScript&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAB4AAAAeCAYAAAA7MK6iAAADEklEQVRIS+2VW0hUYRSFv3PmnOOMTo4zzZiWlpbdIC0so6ArVERRDwVhUFBBEVEPRfmQ+BZEEb0VBSGE9NJDRFQSUhRJmVlBlN0vjnbRyUvq6NxPnH/UamLswmQ9uF9/+Ndea6+9tsT+Op1/UNIw8FCpPiz1UCnNsNTDUv81Bf4/czksJnLtSShyfNKBsE5Ih5etPnqD0VujmSTGOzQmjTQTiujUt/ho/BwgHHOK4jIuzrdzfPVYVJMUF9n4yxuIsKriBXXvehlrUylbnMnaaXbRgFHdgQgnaj0cq2nB4w0P/BUXeMMMB+VrxqGaBqFsfOwPs7T8OS/a/JQuzGD3vIwfGg1HdPZWNnGq7pNoxKi4wNk2lcLRych9hDcXOlmSl4pFlSmpbOJVmx8dnVAEbjV047IqXNk0kWybxm23l4M3PmDVZHbNTeeTN0TZ1fc8avYR6ZP8l811dEUWW2c5sSaZKDpez713vXw7tvxRZmq2T0FTZCoetLLjghtfSCdzhEKXPzLAtF+OhAFPciZxcWMeE51mHjf3crT6I1WvuvB4Q6KB2EoYsLEF++aPYuecdKFKlz/M9dddnH3UJqR3dwQJ9us82IxjO/yZ1IYVJjg0ShZksDQvlTGpmtgIw3yVzz9z7I6HGrcXf99eJYxxf6PpKQorJ9tYlpdKUXayMJtmkgV7Y+5PPT7hjYQDR1cFXCkK83OsbJnpZFGuFbMqs/VcA2cetuEP6YkDHqHJzMuxiuC4/KxTzNNoYFuRk9LFmYL5gWvvOXSzWTg8IYyNfd05x0VxgUPE5On7rVQ3dIsmDMbr8u2kWRT2XGrk5F0PPcEEMbYoEptnjuTw8iwxz5buII2dAUySRK5dw5Gs4G4PsP7sa2qbekSIJISxIalhquICO9tmu5jqMiNJ0ciL6DoPP/RwpLqF8/UdeIM/iczYdZqeYWG8IwlVhqqXnbT7oh98TSJI0WTGpWlMcZnJSdPE05v2AE88Pt52+Acu2KBZHQtsHBvZYCFBKGykdPxSZQnVFH0PhvkuOH47MgfB+aOnL4fQl0Qz73cKAAAAAElFTkSuQmCC&color=0075C5) — Typed JavaScript superset
- **Sass** ![Sass](https://img.shields.io/github/package-json/dependency-version/eksplein/website/dev/sass?filename=package.json&label=Sass&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAB4AAAAeCAYAAAA7MK6iAAAFJklEQVRIS8VXa2xTZRh+vtOztjtbN9i6rRQ2XMdgThTZBgQZwQWJAbIFRFGJ0QTdxX9Eo4kxmgVjYowk/lDcRTGKkQgY3VAIYNLoZgZhY46Jgc0Wto61G+3Yte26c85nvgPrTreedcQffP+a877v816e91KCB/RILNzq6mput8dmlkL+HJkn20FRwhGSS4E0pkuA2zKl3SCwcyI9o9MLjp8sTm91dbU8n+15gZv3f2WK5+UqgO4B8DgAQwxHJwH8BZAfAyJXU3zk1TEteU3gttfqH6Mc/Z4Aj8TKivKd3DNFqfKTAleJTPYVfll+JZr+HODjzx3X5aSMvAhKPwSQpQnKEcRbUyBkpkJvNkEXrwcoxdRoAAGXD2NdblBR6gUh7zqGko/tPbFXUtuaA9xWWf8SKK0FIGiBmvKssO5apwATngPRcTOilEIWZYz/68HNI3aIowE/CKksrC3/ThOYpRccPRUtUi5Oh6TVWUgryYdp5RJIkyICfT74e7zw99xGyDsOWZJgSE1E8pqHsLgwGwPnO9H/8yWG1wuZlKrTHo6YEcnISy3Ramowm5C5bxMScyzgjHEY7riJgbNXMOkZhugPKSlWP+akrWobdAkGXP+oIVzzoKjbOE04BZi1TGn/0jcB+vHs9CbY0rH8lS0wZiTD3zcEd2MrRq70xuRbdsVWxCUJ6PqEJXD6kbdPWW8dYq2mAHdUfZsuysFGABvUFlktGah+UQIG7Vcx+FsnQkPjMUEN6cnIe2eXosMcVb2LPGcsW1Pz8qACfHl/zUbKc3Z1n+pTEpF7YAf0aUnwnG6H+1RbTEAmoBMMsL2+Dcb0JDg+Pwd/r1etN0lEuaTgSBUrKdBaUXuQgLynlsh4eo3C3AnnAJxfnIc4HowJLGSZYdlZgMRcC26dvABfS/ec+lPQD4rqKt+/C1xe20QIKVZbXvlWKRJXWNBz9A/4mq+HPyWtzlSMjf7Tp0wJ9lh2LDvWYvG6HMjBKdz82o6x6+45oEyWUtpcVF+5+W6qK+o8FMhQA+e+sROmVVY4Dp/DSEfPDD0IwZKyQqUEhCNK/Rl7We0nHAPwtXQh5NPmAQEGCuoqLApwW0UdmyqqKQCkbcnHsuc34s4lB3q++R1UnmkZNjDikgVwBl6JWp6SIE0EIQWnYpYDgFxYV6HTBGYkYYw2rVqCvh9aMHSxOwJ8IQgaMjPA0VLNlHiTEZbta7G4yIahlm54zrQvNCpNvyJSHY1c4ZbnCMybH8bSPesx0ulC79EmyKKk1FchiyiDyvOu3ggnIsgVrZ1muywsNyNzXzHkYAjiBFu7AOF1bAlguM2J0Wv9UVk8205EO0UbIGoFtoGsZYVI2ZQHPl4P9y+Xcdt+FeCI0nLLnt2AvpMXI9ivkevIAaI1Mpkym9XW3eshT07B3diGzBeegN/lg+vYn2HbrI/NT+bD3dAKKs2b9siRqbUkEnIyYCvfiuH2G+hvaIUcEpF/cK8SWd+JC2Fgto1Si/PgbbrGlv88hJ+1JJjk7LXI6XllVvNJ8ej+9DRC3jGwqWWrekrp6zuXnGGA+MxUxC0SMNrp0gRlp9CctTgtrT4E2A5ecWAHJP8kug79isSVFix9ZgPEsQCch89FtBVzThwLzkcu7UMgDH7v9NEJBoFFLGSlhgfHxI1BOD47CykQup/5Efv0YdbUx15CdnpW8qNZCnuDnmGM/u1a0JZSebXwY0+d9vB5y2ZF5HUTM+L7Pm/VFh/IQT/twAP5CxMzn/9D4D8kI209RpJ4jAAAAABJRU5ErkJggg==&color=CD669A) — CSS superset with nested rules and variables

#### DevOps

- **Docker** ![Docker](https://badgen.net/scoop/v/docker?label=Docker&icon=docker&color=319DEC) — Application containerization
- **Github Actions** ![Github Actions](https://badgen.net/badge/Github%20Actions/Ubuntu%2018.04/DD4814?icon=github) — Continuous integration
- **Drone CI** ![Drone](https://img.shields.io/badge/Drone%20CI-1.8.1-253D5F?logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFn0lEQVR4XtWbjXUcNRCAZyogVACugKQCSAXEFQRXQFIBcQUkFZBUELsC7AqIK8CpAKhgeN8+6d7enqSRdqVNTu/dyzn7I82n0fxJpzK4mdn3IvKDiDwNnyciwoe/5+1RRPj8KyKfwvd7VeX/hjUd8WYz+1lEfhKRFyICgC0NAHci8kFV+bdr6wYgzPRLEfmlg9A5IYHxXkTeqSqasrn1BIDgf2weUf0LAHG9dYl0A8C4zWxvCGjBW1W9rud2fGdXAF8IAt2yNC5VFePZ1JoBmNlLVf1Q6uULaEIczitVfddCoBqAmeG6fg9G7r2qXn2lELANr2uNZBWAIPyfC9/9NUNgKTyvgeACyAgfJ//sIRQBOML3hvBfiADnK4tlRxS5trma4AH4GKI5bwBrNeE2BDafSv7czAibiSr5tAK5UdXLnABZAGb2Khg9T/g1moClflOzRpedmxkh9hsR+bF2YMEovk3dnwQQiP/V0EELhCdrBE+AIOhCqG8qx/ksFSfkAPy9IZ53l0PlgN3bwkTh9mqWxaOqXixfegLAzFCv39zeyzfsCQFDibH7rmLM5A7Id2hHAEJGh+rz0q1tTwgYSVJlbzmQO7AUDjWGJQDUiZS2VyNtxZgWW1DlWEOIxRIEYsCxFlBMf80MD4HX8hp1BezH1A4Ags//x3u68fqVqgI12YLghNdY9lKryvrMDFg13uEiasEcQI+1Pxfis6pmq0FhxqgftCy3YmATXCQhu9cOtmAOgNlvGYzXCQlJ0veGmWega/ojaHpW0CogeV7h4BEmAA3kPKHn1w9qtnzIzLa4WV53Ys1jHw1ejGTpLgJgpn5tkc6590FVl1Xf6ZFOtYITaz4DQL81QdxkDCOArTOy5HFkaecXGwyVNx9ZA2tm5j1MFYnASIPvB0DPVlLRmsHVjOVWVXF9J83M8PM1gdEEoNZ/1gwq3pME0NnWsGmSdJ8NWnYFgN7uDwiTgUkYPwZc46aqYKtqLpepjQeuAXAjIkRhPVvSBXbWgJKhrQVwC4Dam1sA5ZYAfr9XtNljCdwDAJeRdFktEi/uzeYADQbK675kaGuDukcA9LLK8wFnozUz6xVzJAscrV5tFABgfJuq/ISkCzflpa4lDSipf5NXGwmgFKhs2UOkevw0V0RtNeojAdyp6vPcNJrZmtoDwr/InRNYk9IDoDZq8oxS6noyHog3hrygtrD5OQif3QBdEdM8jHKDUcaiFnBTmDWqRiyLVPh6z95BqbAye0+ri53c4IhAaK4NWXeViBSPzg61HolZkWlOgdCIUHgpW7E0tmZtFWxLi4GdQuGWB9aOlfwdCGjb8NYg0+WodDgnJMdZXg8nUF94uYgFkZGeYCkvuQd2YfWRN06phFigCNPRhKloGwGs8clbJxIA9Ethwz3yFkJcslY8Rqw2u5svBQhTvhIBNIWPWyVPPI9vjxshXOZ7LHbEE6a5EvtaCByqupmXxfdcBr0ZtkI47FmM3BjpLaT3vhYIyY0RVKx3cdQbdO/rtRCIUKcN0tGbo70FrHmfC2H+ktT2+LlrAfJVQxh1QKJmpkbfUwUhV1Y+Z48wB+tCyAHoWr8fPdXO+4sQSsfk9sgS92KTzUa9g5KjawV7AMhu1J64weVoQrWGsNQ7cLCHIGv6yO4exZfVHpY+RwgP5BNeouUCgNQZakKV8O4SmOtcgEAFt+cxujVq7T3Dr1n45YibYjcBiL2uOETtDbjn9ezBrFwnVUsgYRzJ0fEQNacwegqYe5e7Z9AVwEwbiBWo0GzZ59sCiJ0i6oxH539bXrhKAxa2gTSaAVBV2hNE01ofogEJI1na4WmZmJKqU0dk1quMnNfpZg1IdRCOwrDfQE6x1U6wvrE3/PRldSV5uAbkOgjVXECwVDCebH/xfQkG382sxp/PIyxn+Yb+fP5/z2CtqoIeQP8AAAAASUVORK5CYII=) — Continuous delivery

<br>

<img src="https://eksplein.me/excalidraw/stack.png" />

<br>

## Contributing

> Since this is pretty much early concept, contributing guides are still work-in-progress.

_**If specifically contributing to the client** →_ [`eksplein/website/CONTRIBUTING.md`](https://github.com/eksplein/website/blob/master/CONTRIBUTING.md)

During development, the required tools are **Rust**, **Node** and **Redis** (or **Memurai**, if working on Windows). A decent **Markdown** editor (_e.g._ **Typora**) is recommended, if contributing on blog posts.

Then you shall proceed with the **[CONTRIBUTING](https://github.com/eksplein/eksplein/blob/master/CONTRIBUTING.md)** guide.

##### TL;DR

Make sure the Redis / Memurai database is up

```bash
redis-cli ping     # With Redis
memurai-cli ping   # With Memurai, if using Windows
```

Run the server

```bash
cargo run
```

Display CLI usage guide

```bash
> cargo run -- --help
EKSPLEIN v0.0.1
usage: eksplein [-h] [-v] [-d PATH] [-p PORT] [-u URI]
  --help, -h     —  Show CLI usage guide
  --version, -v  —  Show CLI version
  --dist, -d     —  Set frontend dist folder (default: dist/)
  --port, -p     —  Set server port (default: 7000)
  --uri, -u      —  Set Redis URI (default: redis://127.0.0.1/)
```

Assuming the server port value is **7000**, GraphQL Playground can now be accessed via `http://localhost:7000/graphql`.

## Inspirations

- [DigitalOcean Community tutorials](https://www.digitalocean.com/community/tutorials) — DigitalOcean
- [whatthefuck.is](https://whatthefuck.is/) — Dan Abramov
- [Urban Dictionnary](https://www.urbandictionary.com/) — Urban Dictionary LLC

## License

Actix-based Eksplein **server** is under [**GPLv3 License**](https://github.com/eksplein/eksplein/master/LICENSE). ![License: MIT](https://img.shields.io/badge/License-GPLv3-blue.svg)

Sapper-based Eksplein **client** is under [**GPLv3 License**](https://github.com/eksplein/website/master/LICENSE). ![License: MIT](https://img.shields.io/badge/License-GPLv3-blue.svg)

Markdown-based Eksplein **posts**, code snippets not included, are, unless stated otherwise by the said post author, under the [**CC BY-SA License**](https://github.com/eksplein/eksplein/blob/master/client/LICENSE-POSTS.md). ![License: CC BY-SA 4.0](https://img.shields.io/badge/License-CC%20BY--SA%204.0-lightgrey.svg)

