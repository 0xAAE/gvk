# Query URI
```
https://oauth.vk.com/authorize?client_id=7720259&display=page&redirect_uri=https://oauth.vk.com/blank.html&scope=offline&response_type=token&v=5.52
```

# Log

### authentication succeeded

#### initial request
```
LoadEvent::Started: https://oauth.vk.com/authorize?client_id=7720259&display=page&redirect_uri=https://oauth.vk.com/blank.html&scope=offline&response_type=token&v=5.52
LoadEvent::Committed: https://oauth.vk.com/authorize?client_id=7720259&display=page&redirect_uri=https://oauth.vk.com/blank.html&scope=offline&response_type=token&v=5.52
LoadEvent::Finished: https://oauth.vk.com/authorize?client_id=7720259&display=page&redirect_uri=https://oauth.vk.com/blank.html&scope=offline&response_type=token&v=5.52
```
#### redirect to auth page
```
LoadEvent::Started: https://oauth.vk.com/authorize?client_id=7720259&display=page&redirect_uri=https://oauth.vk.com/blank.html&scope=offline&response_type=token&v=5.52
LoadEvent::Redirected: https://oauth.vk.com/authorize?client_id=7720259&redirect_uri=https%3A%2F%2Foauth.vk.com%2Fblank.html&response_type=token&scope=65536&v=5.52&state=&display=page&__q_hash=1c2137ecbe5c8d89ab654234c4a00d69
LoadEvent::Redirected: https://login.vk.com/?act=grant_access&client_id=7720259&settings=65536&response_type=token&group_ids=&token_type=0&v=5.52&display=page&ip_h=f370278ac4c11336ec&hash=1610265045_9f2324ac1dfa79668b&https=1&state=&redirect_uri=https%3A%2F%2Foauth.vk.com%2Fblank.html
```
#### redirect after auth succeeded
```
LoadEvent::Redirected: https://oauth.vk.com/blank.html#access_token=cfe61d4f475888fdd0c41f9da8459f6daae58aa9b311d46c4a99be59124641d25e02dbe31c50492b4a0fa&expires_in=0&user_id=184946538
LoadEvent::Committed: https://oauth.vk.com/blank.html#access_token=cfe61d4f475888fdd0c41f9da8459f6daae58aa9b311d46c4a99be59124641d25e02dbe31c50492b4a0fa&expires_in=0&user_id=184946538
LoadEvent::Finished: https://oauth.vk.com/blank.html#access_token=cfe61d4f475888fdd0c41f9da8459f6daae58aa9b311d46c4a99be59124641d25e02dbe31c50492b4a0fa&expires_in=0&user_id=184946538
```
### authentication failed then succeded

#### initial rewuest
```
LoadEvent::Started: https://oauth.vk.com/authorize?client_id=7720259&display=page&redirect_uri=https://oauth.vk.com/blank.html&scope=offline&response_type=token&v=5.52
LoadEvent::Committed: https://oauth.vk.com/authorize?client_id=7720259&display=page&redirect_uri=https://oauth.vk.com/blank.html&scope=offline&response_type=token&v=5.52
LoadEvent::Finished: https://oauth.vk.com/authorize?client_id=7720259&display=page&redirect_uri=https://oauth.vk.com/blank.html&scope=offline&response_type=token&v=5.52
```
#### redirect to auth page
```
LoadEvent::Started: https://oauth.vk.com/authorize?client_id=7720259&display=page&redirect_uri=https://oauth.vk.com/blank.html&scope=offline&response_type=token&v=5.52
LoadEvent::Redirected: https://oauth.vk.com/authorize?client_id=7720259&redirect_uri=https%3A%2F%2Foauth.vk.com%2Fblank.html&response_type=token&scope=65536&v=5.52&state=&display=page&m=4&email=avramenko.a%40gmail.com
LoadEvent::Committed: https://oauth.vk.com/authorize?client_id=7720259&redirect_uri=https%3A%2F%2Foauth.vk.com%2Fblank.html&response_type=token&scope=65536&v=5.52&state=&display=page&m=4&email=avramenko.a%40gmail.com
LoadEvent::Finished: https://oauth.vk.com/authorize?client_id=7720259&redirect_uri=https%3A%2F%2Foauth.vk.com%2Fblank.html&response_type=token&scope=65536&v=5.52&state=&display=page&m=4&email=avramenko.a%40gmail.com
```
#### redirect after auth failed
```
LoadEvent::Started: https://oauth.vk.com/authorize?client_id=7720259&redirect_uri=https%3A%2F%2Foauth.vk.com%2Fblank.html&response_type=token&scope=65536&v=5.52&state=&display=page&m=4&email=avramenko.a%40gmail.com
LoadEvent::Redirected: https://oauth.vk.com/authorize?client_id=7720259&redirect_uri=https%3A%2F%2Foauth.vk.com%2Fblank.html&response_type=token&scope=65536&v=5.52&state=&display=page&__q_hash=9b390536daa583dd0018ca40054dba3c
LoadEvent::Redirected: https://login.vk.com/?act=grant_access&client_id=7720259&settings=65536&response_type=token&group_ids=&token_type=0&v=5.52&display=page&ip_h=f370278ac4c11336ec&hash=1610266534_80c035c32d3269b9fe&https=1&state=&redirect_uri=https%3A%2F%2Foauth.vk.com%2Fblank.html
```
### redirect after auth succeeded
```
LoadEvent::Redirected: https://oauth.vk.com/blank.html#access_token=05c2927b1f7df45585e99761c2f99689d95331d6a1a074d5f5b9e71c7c263aea01693d890e4c826cf005e&expires_in=0&user_id=184946538
LoadEvent::Committed: https://oauth.vk.com/blank.html#access_token=05c2927b1f7df45585e99761c2f99689d95331d6a1a074d5f5b9e71c7c263aea01693d890e4c826cf005e&expires_in=0&user_id=184946538
LoadEvent::Finished: https://oauth.vk.com/blank.html#access_token=05c2927b1f7df45585e99761c2f99689d95331d6a1a074d5f5b9e71c7c263aea01693d890e4c826cf005e&expires_in=0&user_id=184946538
```
