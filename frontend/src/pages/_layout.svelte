<style>
    #main-navbar{
        display: flex;
        flex-direction: row;
        align-items: center;
        height: 5rem;
        margin: 0px;
        padding: 0px 20px;
        box-shadow: 0px 0px 5px #6B7280;
        background-color: #F9FAFB;
        z-index: -1;
    }

    #navbar-brand{
        flex-grow: 1;
        font-size: 25px;
        font-weight: bold;
    }

    #main-container{
        padding: 16px;
    }
</style>

<script>
    import {getFetch, postFetch} from "../ajax.js";
    import Cookies from 'js-cookie';

    async function determineAuthenticationStatus(){
        let response = await getFetch('/api/accounts/status');
        if(response.ok){
            return {
                signedin: true,
                username: (await response.json()).username
            };
        }else if(response.status === 401){
            return {
                signedin: false
            };
        }else{
            throw new Error(`${response.status} ${response.statusText}`);
        }
    }
    let authenticationPromise = determineAuthenticationStatus();

    async function signOut(){
        let response = await getFetch('/api/accounts/signout');
        if(response.ok){
            Cookies.remove('actix-session');
        }else{
            alert(`${response.status} ${response.statusText}`);
        }
        window.location.reload();
    }
</script>

<div id="main-navbar">
    <div id="navbar-brand">
        <a href="/" style="text-decoration: none; color: inherit;">Smit</a>
    </div>
    <div id="navbar-credentials">
    {#await authenticationPromise}
        Loading...
    {:then authenticationStatus}
        {#if authenticationStatus.signedin}
            {authenticationStatus.username}
            <a class="smit-button" style="background-color: #DC2626" on:click={signOut}>Sign out</a>
        {:else}
            <a href="/signup" class="smit-button" style="background-color: #2563EB">Sign up</a>
            <a href="/signin" class="smit-button" style="background-color: #059669">Sign in</a>
        {/if}
    {:catch error}
        {error.message}
    {/await}
    </div>
</div>

<div id="main-container">
    <slot/>
</div>