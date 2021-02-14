<script>
    import FormHost from './_components/FormHost.svelte'
    import FormRow from "./_components/FormRow.svelte";

    let email, username, password, confirmPassword;

    function signUp(){
        alert(`${email} ${username} ${password} ${confirmPassword}`);
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