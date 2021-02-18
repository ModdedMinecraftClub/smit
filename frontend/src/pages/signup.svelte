<script>
    import FormHost from './_components/FormHost.svelte'
    import FormRow from "./_components/FormRow.svelte";
    import {postFetch} from "../ajax.js";

    let email, username, password, confirmPassword;

    async function signUp() {
        if (password !== confirmPassword) {
            alert("Passwords don't match.");
            return;
        }

        let response = await postFetch('api/accounts/signup', {
            email: email,
            username: username,
            password: password
        });

        if (response.ok) {
            alert("You have successfully registered. Check your email for a verification link before you sign in.")
            window.location.href = '/signin';
        } else {
            let text = await response.text();
            if(text.length === 0){
                alert(`${response.status} ${response.statusText}`);
            }else{
                alert(text);
            }
        }
    }

</script>

<style>
    .smit-button{
        margin-top: 1rem;
    }
</style>

<FormHost>
    <span slot="fields">
        <FormRow title="Email: " type="email" bind:value={email}/>
        <FormRow title="Username:"  type="text" bind:value={username}/>
        <FormRow title="Password: " type="password" bind:value={password}/>
        <FormRow title="Confirm Password: " type="password" bind:value={confirmPassword}/>
    </span>
    <span slot="buttons">
        <a href="#" class="smit-button" style="width: calc(100% - 25px); background-color: #34D399" on:click={signUp}>Sign up</a>
        <a href="/" class="smit-button" style="width: calc(100% - 25px); background-color: #7C3AED">Cancel & Return To Home</a>
    </span>
</FormHost>