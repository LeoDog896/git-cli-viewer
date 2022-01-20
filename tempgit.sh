#!/bin/sh

# Create a new temp directory
tmp_dir=$(mktemp -d -t git-$(date +%Y-%m-%d-%H-%M-%S)-XXXXXXXXXX)

if [ "$#" = "1" ]
then
	githubDiscoveryURL=$1
else
	if [ "$1" = "github" ]
	then
		githubDiscoveryURL="https://github.com/$2"
	elif [ "$1" = "gitlab" ]
	then
		githubDiscoveryURL="https://gitlab.com/$2"
	elif [ "$1" = "git" ]
	then
		githubDiscoveryURL="$2"
	fi
fi

git clone $githubDiscoveryURL $tmp_dir --depth=1

(cd $tmp_dir; eval $SHELL)

rm -rf $tmp_dir
