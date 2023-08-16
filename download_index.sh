#!/bin/bash

mkdir search_index
bash ./gdownload.sh "1FoXDuTb5vJeUTCeSetDuYosSKCCWeNjI" search_index/ordered_titles.zip;
bash ./gdownload.sh "1cADzNona171CcJWcIo_DZu8KlXIDqftT" search_index/test_links.zip;
bash ./gdownload.sh "1_wVJ_IRhbgs3Q2sxWjPQRuYKvQG4ODb_" search_index/incoming_links.zip;
