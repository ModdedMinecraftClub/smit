<script>
    export let title;
    export let type;
    export let value;

    function onInput(event){
        value = event.target.value;
    }
</script>

<style>
    #form-row{
        display: flex;
        flex-direction: row;
        align-content: center;
        align-items: center;
        padding-bottom: 1rem;
        flex-wrap: wrap;
    }

    #label{
        padding-right: 0.5rem;
    }

    #input{
        flex-grow: 1;
        min-width: 0;
    }
</style>

<div id="form-row">
    <label for="input" id="label">
        {title}
    </label>
    <input id="input" type={type} on:input={onInput} on:change={onInput}>
</div>