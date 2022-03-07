### Some examples about axum_start

#### Post_articles

* start the server
> cargo run --example post_articles

* open another terminal (below is to use the httpie)
> -- Login to get an token:
> 
> http post "http://localhost:8888/login" user_id=100 name=test
> 
> -- Post an article by Token
> http post "http://localhost:8888/post" title="new article" "Authorization: Bearer <<token>>"
> 
> -- Query articles
> http "http://localhost:8888/post" "Authorization: Bearer <<token>>"

