<script>
    import FormHost from './_components/FormHost.svelte'
    import FormRow from "./_components/FormRow.svelte";

    let email, password;

</script>

<style>
    .smit-button{
        margin-top: 1rem;
    }
</style>

<FormHost>
    <span slot="fields">
        <FormRow title="Email: " type="email" bind:value={email}/>
        <FormRow title="Password: " type="password" bind:value={password}/>
    </span>
    <span slot="buttons">
        <a href="#" class="smit-button" style="width: calc(100% - 25px); background-color: #34D399">Sign up</a>
        <a href="/" class="smit-button" style="width: calc(100% - 25px); background-color: #7C3AED">Cancel & Return To Home</a>
    </span>
</FormHost>