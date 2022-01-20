#!/bin/sh

# Create a new temp directory
tmp_dir=$(mktemp -d -t git-$(date +%Y-%m-%d-%H-%M-%S)-XXXXXXXXXX)

if [ "$#" == "1" ]
then
	GithubDiscoveryURL = $1
else
	if [ "$1" == "github" ]
	then
		GithubDiscoveryURL = "https://github.com/$2"
	elif [ "$1" == "gitlab" ]
	then
		GithubDiscoveryURL = "https://gitlab.com/$2"
	elif [ "$1" == "git" ]
	then
		GithubDiscoveryURL = "$2"
	fi
fi

git clone $GithubDiscoveryURL $tmp_dir --depth=1

# Clone the repository
git clone https://github.com/Project-Cepi/Sabre $tmp_dir --depth=1

(cd $tmp_dir; eval $SHELL)

rm -rf $tmp_dir