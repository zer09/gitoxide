### Cloning
* **git-protocol**
  * [x] sideband-all support
  * [x] test for multi-stage fetch response (one without a pack file in first round)
  * [x] delegate interaction to support clone
    * [x] parse server negotiation response
    * [x] negotiation via delegate
    * [x] received pack file passed to delegate
    * [x] assure there is a way to do fetches with have/want negotiation
* **gixp-pack-receive**
  * [x] hookup `git-protocol` with delegate to allow for receiving full packs
  * [x] json support
  * [x] may optionally write received refs to the specified directory
  * [x] journey tests for each connection method
    * [x] file
    * [x] git
    * [ ] ~~ssh~~
    * [x] https (unauthenticated)
    * [ ] ~~https (authenticated)~~
  * [x] make cloning https://github.com/torvalds/linux work - it's probably about not sending too many wants and filtering refs to send

### NEXT ITERATION: Fetching _(more analysis needed after previous block)_

* **git-transport**
 * [ ] a way to support shallow lines during V1 handshake (doesnt' seem to happen in V2 at all)
* **prodash**
  * [ ] force sub-progress support for `Send` and `'static` bounds
* **git-ref**
  * [ ] create ref pointing to ID
      * _assure to keep the path towards symbolic refs open, and allow specifying if these should be followed or not_
* **git-repository**
  * [ ] instance for a valid looking repository
    * [ ] support shallow repos/references
  * [ ] create-update refs as received from clone/git-receive-pack
* **gix clone**
  * [ ] try initializing repo on output path - if so, use that to learn about pack location and place new pack there, allow Repo to create refs somehow.
    * _probably this is done using the repository itself, which steers the whole process and injects it's own delegates.
  * [ ] otherwise create the scaffolding needed for a new repository, probably based on `init` implementation
* **git-daemon-proxy** - obtain real-world examples for the V1/V2 git protocols
  * a standalone tool to serve as `git-daemon-proxy`, which intercepts all interactions with it and dumps each one
    into files like N.request and N.response.
  * Differentiate request and response delimiters using timeouts
* **receive pack**
  * [ ] resolve thin pack with Bundle
* **git-repository**
  * [ ] clone from https remote
  
### FSCK an entire repository

* **multi-db** (incorporate object lookup for loose objects and packs)
  * [ ] single threaded
  * [ ] optional object cache
  * [ ] fs-check - verify all object content of a git repository
  
### Notes

* response-end packets of V2 are [not sent in stateless-rpc mode](https://github.com/git/git/blob/master/serve.c#L246:L246) (i.e. http mode) and maybe even V2 daemons
* protocol V2 [is always stateless](https://github.com/git/git/blob/master/builtin/upload-pack.c#L54:L54)
* The negotiation is the meat of the fetch algorithm, and even though not important for clones, the back-and-forth seems very relevant 
  to how the transfer interface should be built. I feel it must be on packet line level.
  * after having looked at the http implementation and actual HTTP chatter, there is no need for packet line level in `git-protocol` after all.
    All good.

### Other

To be picked in any order….

* **git-odb**
  * Find errors with `Box<dyn std::error::Error>` fields and see if these could be generic with the use of `thiserror`
    * [ ] [definitely here](https://github.com/Byron/gitoxide/blob/8f8d14f4606e99dc710eb352a985db48c00ea4f4/git-odb/src/pack/index/traverse/mod.rs#L142)
    * [ ] _…keep looking…_
* **prodash**
  * [ ] finish transitioning to futures-lite to get rid of futures-util dependency to reduce compile times
* **criner**
  * [ ] upgrade to prodash 9.0
  * [ ] switch to `isahc`
    seems to allow async-reading of bodies, allowing to get rid of reqwest and tokio. Redirect is configurable.

[josh-aug-12]: https://github.com/Byron/gitoxide/issues/1#issuecomment-672566602
