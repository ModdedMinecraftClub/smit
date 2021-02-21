import {issueid} from "./pages/issue/[issueid].svelte";

export async function postFetch(url, json){
    return await fetch(url, {
        method: 'post',
        headers: {
            'Content-Type': 'application/json'
        },
        credentials: 'same-origin',
        body: JSON.stringify(json)
    });
}

export async function getFetch(url){
    return await fetch(url, {
        method: 'get',
        headers: {
            'Content-Type': 'application/json'
        },
        credentials: 'same-origin'
    });
}

export async function getFetchJson(url){
    let response = await getFetch(url);
    if(response.ok){
        return await response.json();
    }else{
        throw Error(`${response.status} ${response.statusText}: ${await response.text()}`)
    }
}