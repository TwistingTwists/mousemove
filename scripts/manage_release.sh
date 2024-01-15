# set -eou 

export $(cat .env | xargs)

list_release(){
    curl -L \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer $GH_TOKEN" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/$OWNER/$REPO/releases
}

list_release

create_release(){
    curl -L \
  -X POST \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer $GH_TOKEN" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/$OWNER/$REPO/releases \
  -d '{"tag_name":"v0.2.1","target_commitish":"main","name":"v0.2.1","body":"for windows pre compiled","draft":false,"prerelease":false,"generate_release_notes":false}'
}

upload_release_asset(){ 
curl -L \
  -X POST \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer $GH_TOKEN" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  -H "Content-Type: application/octet-stream" \
  "https://uploads.github.com/repos/$OWNER/$REPO/releases/$RELEASE_ID/assets?name=mousemove.exe" \
  --data-binary "@$BINARY_FILE"

}

# create_release <<< this will return a release_id like --- 137102270
export RELEASE_ID=137102270
# export BINARY_FILE=target/x86_64-pc-windows-gnu/release/mousemove.exe
upload_release_asset # use RELEASE_ID here.