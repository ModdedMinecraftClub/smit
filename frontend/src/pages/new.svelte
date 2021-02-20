<script>
    import FormHost from './_components/FormHost.svelte'
    import FormRow from './_components/FormRow.svelte';
    import TextAreaFormRow from './_components/TextAreaFormRow.svelte';
    import {postFetch} from "../ajax.js";

    let title, markdown;

    async function post(){
        let response = await postFetch('/api/issues/new', {
            title, markdown
        });
        if(response.ok){
            let json = await response.json();
            window.location.href = `/issue/${json.uuid}`;
        }else{
            alert(await response.text());
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
        <FormRow title="Title: " type="text" bind:value={title}/>
        <TextAreaFormRow title="Markdown: " type="password" bind:value={markdown}/>
    </span>
    <span slot="buttons">
        <a href="#" class="smit-button" style="width: calc(100% - 25px); background-color: #34D399" on:click={post}>Post</a>
        <a href="/" class="smit-button" style="width: calc(100% - 25px); background-color: #7C3AED">Cancel & Return To Home</a>
    </span>
</FormHost>