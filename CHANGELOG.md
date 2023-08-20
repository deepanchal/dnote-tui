# Changelog

## 0.1.0 (2023-08-20)


### Features

* **app:** add page_info state to app, add get_page_content method ([a8b08a9](https://github.com/deepanchal/dnote-tui/commit/a8b08a92d85d745ed281e7e96beefb69998d177f))
* **app:** add stateful pages list, add getters for books & pages ([a99f23f](https://github.com/deepanchal/dnote-tui/commit/a99f23fd71af0194cbca6e7a7a764cc5977b6fb1))
* **app:** add StatefulList, add books list to App, load books from dnote in constructor ([1df5378](https://github.com/deepanchal/dnote-tui/commit/1df53787209f36d0b887bbca5525461c0208e3d1))
* **app:** add TuiChunk enum, add selected_chunk property to app, add select_next_chunk & select_prev_chunk methods ([38ee7e8](https://github.com/deepanchal/dnote-tui/commit/38ee7e884edb50f7549fd371070def331efaf057))
* **app:** remove test counter & related methods from app ([5e83ed0](https://github.com/deepanchal/dnote-tui/commit/5e83ed095b4e6ac43458f8536493260c134449b6))
* **bin:** add dnote_client_test binary w/ test usage from main ([3d4b380](https://github.com/deepanchal/dnote-tui/commit/3d4b380b4d26c6b3a81a95c133e2fb424e8a8c32))
* **cargo:** set default-run to dnote-tui ([47da791](https://github.com/deepanchal/dnote-tui/commit/47da79149b016172df496f7605e7d77056294ba6))
* **deps:** add tui using ratatui & crossterm deps, update lockfile ([d0a6622](https://github.com/deepanchal/dnote-tui/commit/d0a6622681ae9df6e82441213f76a40e0a502d88))
* **dnote_lib:** comment print statements in DnoteClient methods ([e425914](https://github.com/deepanchal/dnote-tui/commit/e4259146e6166adbb5b27c5fb6e22dde1e05519b))
* **handler:** add keybindings for navigating through books list ([40305e5](https://github.com/deepanchal/dnote-tui/commit/40305e59a7f07de644bcead799f307046910085b))
* **handler:** handle navigation using selected_chunk & helper methods ([1b7e5f8](https://github.com/deepanchal/dnote-tui/commit/1b7e5f8f525a36e983a94e13d3cb35992040f953))
* init with binary project ([6cbeb5f](https://github.com/deepanchal/dnote-tui/commit/6cbeb5f324af57fdc0a6d5eced6e743627df8e79))
* **lib:** add DnoteClient, DnoteClientError, add funcs for DnoteClient ([c737810](https://github.com/deepanchal/dnote-tui/commit/c7378102641d7b0c71e70c94c18b37f7c9da0f03))
* **lib:** add DnotePageInfo struct ([00b85d7](https://github.com/deepanchal/dnote-tui/commit/00b85d7a77afdd75d44fbf75ff3db90976646190))
* **lib:** add lib.rs ([56e7c63](https://github.com/deepanchal/dnote-tui/commit/56e7c63da8e0d45a1324115cf3fe4833356beb55))
* **lib:** implement FromStr for DnotePage, remove uuid & content fields from struct ([554f602](https://github.com/deepanchal/dnote-tui/commit/554f60259e4ca159c768aa7d379f38268ecd1450))
* **lib:** implement FromStr for DnotePageInfo ([1823c2c](https://github.com/deepanchal/dnote-tui/commit/1823c2cac3f47fd76219216e7a602ea9af76b209))
* **lib:** implement FromStr trait for DnoteBook to parse string into struct ([68c4af8](https://github.com/deepanchal/dnote-tui/commit/68c4af8671f4f40d2fb4b2ef55dc915e27373284))
* **lib:** implement view_page_info func for DnoteClient ([0cdd126](https://github.com/deepanchal/dnote-tui/commit/0cdd126ca180cb97e851c371bd72061ab1667071))
* **lib:** implement view_pages func for DnoteClient, format file ([5554887](https://github.com/deepanchal/dnote-tui/commit/55548874f22747aebf6be1aad3274c05ed9a6d26))
* **lib:** improve DnoteClientError enum, use ? shorthand for view_books ([f06b3e8](https://github.com/deepanchal/dnote-tui/commit/f06b3e82a6638ad0322b4ac34039011a5284a1ff))
* **main:** test out DnoteClient in main fn ([25fdb48](https://github.com/deepanchal/dnote-tui/commit/25fdb48e25782496abbdea5b15021dd3d663006e))
* **setup:** use rust-tui-template to setup files & mods ([3ebf5c4](https://github.com/deepanchal/dnote-tui/commit/3ebf5c4b4d34b31f54111b9a66e62a9194d421f7))
* **src:** add dnote_lib mod w/ DnoteBook & DnotePage public structs ([7ca9652](https://github.com/deepanchal/dnote-tui/commit/7ca965234a101721b2442fb39040a493f85766b7))
* **ui:** adjust areas for chunks ([7808385](https://github.com/deepanchal/dnote-tui/commit/78083853e2f4f2339a88abd7ce16bdf5c14d276e))
* **ui:** render books list in books_block chunk from stateful list of books in app's instance ([85cd6de](https://github.com/deepanchal/dnote-tui/commit/85cd6de5bdb6345338cc9fe4190d56cad2c1bc4a))
* **ui:** render page content in page_content chunk using page_content state ([b7ab6e5](https://github.com/deepanchal/dnote-tui/commit/b7ab6e5b8ec7c2513ce6c3e1894f790c9f654535))
* **ui:** setup layout for books, pages, & content ([9e1d769](https://github.com/deepanchal/dnote-tui/commit/9e1d7690047ac0fbcf11ef1ddcef8dd202231e1b))
* **ui:** use get_books, render pages list for currently selected book ([1635055](https://github.com/deepanchal/dnote-tui/commit/1635055239837311634ae869a87a479deda6f8c8))


### Bug Fixes

* **app:** don't allow next selection when on pages section ([54ed8ff](https://github.com/deepanchal/dnote-tui/commit/54ed8ff7e0b2ff9e2321ac6f94ce751e70aa2fce))
* **deps:** update rust crate crossterm to 0.27.0 ([7e25972](https://github.com/deepanchal/dnote-tui/commit/7e25972d918e200d0ea306020e6b7d16d290ad67))
* **deps:** update rust crate tui to 0.22.0 ([a85cd3b](https://github.com/deepanchal/dnote-tui/commit/a85cd3b9141cb752407410b21f0850590f667ff8))
* **handler:** deselect page before selecting books chunk ([e615765](https://github.com/deepanchal/dnote-tui/commit/e6157653c3c99aab4342a9d83dcf55c95c68159d))
* **handler:** select page on right key handler and then select chunk ([0741da7](https://github.com/deepanchal/dnote-tui/commit/0741da78987d201cfcc4a8650cb08c271a677972))
* **ui:** remove redundant clone call in ui.rs ([e520072](https://github.com/deepanchal/dnote-tui/commit/e520072da3b14300f34bdd6403b06509a5e89cdd))
