# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2023-08-20

### Bug Fixes

- **app**: Don't allow next selection when on pages section ([54ed8ff](54ed8ff7e0b2ff9e2321ac6f94ce751e70aa2fce))
- **deps**: Update rust crate tui to 0.22.0 ([a85cd3b](a85cd3b9141cb752407410b21f0850590f667ff8))
- **deps**: Update rust crate crossterm to 0.27.0 ([7e25972](7e25972d918e200d0ea306020e6b7d16d290ad67))
- **handler**: Deselect page before selecting books chunk ([e615765](e6157653c3c99aab4342a9d83dcf55c95c68159d))
- **handler**: Select page on right key handler and then select chunk ([0741da7](0741da78987d201cfcc4a8650cb08c271a677972))
- **ui**: Remove redundant clone call in ui.rs ([e520072](e520072da3b14300f34bdd6403b06509a5e89cdd))

### Documentation

- **README**: Add README.md from https://github.com/othneildrew/Best-README-Template template ([d04b911](d04b911dbd7361a7d218ae90554c32a06d297973))
- **README**: Remove table of contents from README.md ([c86cabd](c86cabd31dd5ca3ee78ba29117c3a65eb24d4bb6))
- **README**: Make improvements to README.md ([1cb8f36](1cb8f36672619696fdc99e9e6276a33eafe0e76b))
- **README**: Fix license text in README.md ([ef4ce5c](ef4ce5c5b73e392f16faead9e41b2244c768e633))
- **README**: Fix license link ([5ddba61](5ddba6144ff4f577ae4feb974bc77102087eb3d4))

### Features

- **app**: Add StatefulList, add books list to App, load books from dnote in constructor ([1df5378](1df53787209f36d0b887bbca5525461c0208e3d1))
- **app**: Add stateful pages list, add getters for books & pages ([a99f23f](a99f23fd71af0194cbca6e7a7a764cc5977b6fb1))
- **app**: Remove test counter & related methods from app ([5e83ed0](5e83ed095b4e6ac43458f8536493260c134449b6))
- **app**: Add TuiChunk enum, add selected_chunk property to app, add select_next_chunk & select_prev_chunk methods ([38ee7e8](38ee7e884edb50f7549fd371070def331efaf057))
- **app**: Add page_info state to app, add get_page_content method ([a8b08a9](a8b08a92d85d745ed281e7e96beefb69998d177f))
- **bin**: Add dnote_client_test binary w/ test usage from main ([3d4b380](3d4b380b4d26c6b3a81a95c133e2fb424e8a8c32))
- **cargo**: Set default-run to dnote-tui ([47da791](47da79149b016172df496f7605e7d77056294ba6))
- **deps**: Add tui using ratatui & crossterm deps, update lockfile ([d0a6622](d0a6622681ae9df6e82441213f76a40e0a502d88))
- **dnote_lib**: Comment print statements in DnoteClient methods ([e425914](e4259146e6166adbb5b27c5fb6e22dde1e05519b))
- **handler**: Add keybindings for navigating through books list ([40305e5](40305e59a7f07de644bcead799f307046910085b))
- **handler**: Handle navigation using selected_chunk & helper methods ([1b7e5f8](1b7e5f8f525a36e983a94e13d3cb35992040f953))
- **lib**: Implement FromStr trait for DnoteBook to parse string into struct ([68c4af8](68c4af8671f4f40d2fb4b2ef55dc915e27373284))
- **lib**: Implement FromStr for DnotePage, remove uuid & content fields from struct ([554f602](554f60259e4ca159c768aa7d379f38268ecd1450))
- **lib**: Add DnotePageInfo struct ([00b85d7](00b85d7a77afdd75d44fbf75ff3db90976646190))
- **lib**: Implement FromStr for DnotePageInfo ([1823c2c](1823c2cac3f47fd76219216e7a602ea9af76b209))
- **lib**: Add DnoteClient, DnoteClientError, add funcs for DnoteClient ([c737810](c7378102641d7b0c71e70c94c18b37f7c9da0f03))
- **lib**: Implement view_pages func for DnoteClient, format file ([5554887](55548874f22747aebf6be1aad3274c05ed9a6d26))
- **lib**: Implement view_page_info func for DnoteClient ([0cdd126](0cdd126ca180cb97e851c371bd72061ab1667071))
- **lib**: Improve DnoteClientError enum, use ? shorthand for view_books ([f06b3e8](f06b3e82a6638ad0322b4ac34039011a5284a1ff))
- **lib**: Add lib.rs ([56e7c63](56e7c63da8e0d45a1324115cf3fe4833356beb55))
- **main**: Test out DnoteClient in main fn ([25fdb48](25fdb48e25782496abbdea5b15021dd3d663006e))
- **setup**: Use rust-tui-template to setup files & mods ([3ebf5c4](3ebf5c4b4d34b31f54111b9a66e62a9194d421f7))
- **src**: Add dnote_lib mod w/ DnoteBook & DnotePage public structs ([7ca9652](7ca965234a101721b2442fb39040a493f85766b7))
- **ui**: Setup layout for books, pages, & content ([9e1d769](9e1d7690047ac0fbcf11ef1ddcef8dd202231e1b))
- **ui**: Render books list in books_block chunk from stateful list of books in app's instance ([85cd6de](85cd6de5bdb6345338cc9fe4190d56cad2c1bc4a))
- **ui**: Use get_books, render pages list for currently selected book ([1635055](1635055239837311634ae869a87a479deda6f8c8))
- **ui**: Render page content in page_content chunk using page_content state ([b7ab6e5](b7ab6e5b8ec7c2513ce6c3e1894f790c9f654535))
- **ui**: Adjust areas for chunks ([7808385](78083853e2f4f2339a88abd7ce16bdf5c14d276e))- *(No Category)* Init with binary project


### Miscellaneous Tasks

- **build**: Add release-please-config.json ([11b918b](11b918b14ee385ae4edced6e7713ca8417f3fa6a))- *(No Category)* Add screenshot1.png under .github/images
- *(No Category)* Add MIT license
- *(No Category)* Update .gitignore
- *(No Category)* Add .git-blame-ignore-revs
- *(No Category)* Add .gitattributes
- *(No Category)* Add empty CHANGELOG.md
- *(No Category)* Add renovate.json
- *(No Category)* Add issue and pull request templates
- *(No Category)* Add .release-please-manifest.json


### Other

- **main**: Add main workflow for build, lint & test ([708c61b](708c61bfd3f8748bc4d21f08be3da649293b1e86))
- **release**: Add release please action in release.yml ([aae525f](aae525f393b0566fb9c56633fde2fbe83c4d9b69))

### Refactor

- **dnote_lib**: Use ? for handling errors instead of nested match statements ([d7796ca](d7796caf59b51c175949311dea9ea244c198f166))
- **dnote_lib**: Change fn names in DnoteClient ([5d7c1e6](5d7c1e698c5caff3ae27fb79485572c1906b2699))
- **lib**: Use vector collect to improve parsing cmd output to DnoteBook vector ([68f9bed](68f9bed16a4f6b3878a6abf7c224a32cc0c0e63a))
- **ui**: Store chunks in vars & use them to render widget ([71cf60f](71cf60fd7f6a6c262410ad811dfca389a5c2dc85))- *(No Category)* Rename `TuiChunk` to `TuiSection`
- *(No Category)* Replace vars + methods with chunk text to section as it makes more sense
- *(No Category)* Fix git blame ignore revs filename


### Styling
- *(No Category)* Format project with cargo fmt


### Testing

- **lib**: Add tests for parsing implementations for DnoteBook & DnotePage ([c578273](c578273667e141feaa7ee485cb2eceb48da51085))
- **lib**: Add test for parsing FromStr implementation of DnotePageInfo ([e9860c6](e9860c60c8942be7ff6f9e61a23b6d6cef807067))

<!-- generated by git-cliff -->
