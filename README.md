### TODO
## Content
### Song
- [x] List of all available songs
- [ ] Song details / edit view 
- [x] Song deletion
- [x] Song creation view
- [x] Song creation view -> multiple sources
- [x] Song creation view -> multiple covers
- [ ] **Song creation view -> validation on submit** (client side, server may fail)
### Setlist
- [x] Add song to new setlist (from song list view)
- [ ] Add song to existing setlist (with quick search maybe?)
- [ ] Reorder songs in setlist
- [ ] Download setlist as valid printable PDF
- [x] Setlist list view
- [x] Setlist details view
- [x] Setlist deletion
## ACL / Auth
- [ ] User registration
- [ ] User login
- [ ] Limit content visibility to actual logged in user
## Nice to have
- [ ] chordPro format support for storing lyrics
- [ ] chordPro format support for displaying lyrics
- [ ] download chord pro as PDF
- [ ] overwrite chords for song for specific setlist
- [ ] *Add XHR loader indicator*
- [ ] *Add notification popup showing error server response*
- [ ] **Solve all n+1 problems when querying for data**
- [ ] **Handle batch inserting**

### DEVELOPMENT
## Root dependencies
- *Yew* -> https://yew.rs/
- *Actix Web* -> https://actix.rs/docs/whatis
- *sqlx* that is not an ORM -> https://github.com/launchbadge/sqlx#install
- *sqlx cli* (for running migrations) -> https://github.com/launchbadge/sqlx/tree/main/sqlx-cli
- *sqlite*
- *picocss* as I can not do frontend -> https://picocss.com/
## Useful addons
- *mprocs* to run web server and build front app in little bit more convenient way
- *cargo watch* -> https://github.com/watchexec/cargo-watch (to build backend on every change of source file)
## Steps to run application
- install rust :sunglasses: https://rustup.rs/
- install WebAssembly target: ```rustup target add wasm32-unknown-unknown```
- install *trunk* `cargo install --locked trunk`
- navigate to **setlistr-app** directory and run `trunk serve`
### For frontend development you do need only to run setlistrs-server container
- [install docker](https://docs.docker.com/get-docker/)
- ```docker build -f ./setlistrs-server/Dockerfile -t setlistrs-server .```
- ```docker run -p 8081:8081 -v setlistrs-db:/database setlistrs-server```
### If planning to manipulate backend:
- navigate to **setlistrs-server**
- create database `sqlx database create`
- run migrations: `sqlx migrate run`
- run `cargo watch -x run -p setlistrs-server`
### Alternatively you can just assuming you have docker installed you can run everything from *mprocs*
- install [mprocs](https://github.com/pvolok/mprocs#installation)
- run it `mprocs`
- configuration of *mprocs* is self explanatory, to list commands simply `cat mprocs.yaml`
