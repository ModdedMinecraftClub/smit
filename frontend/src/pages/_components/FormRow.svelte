<script>
    export let title;
    export let type;
    export let value;

    function setType(node, type) {
        node.type = type;
    }

    value = "";
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
    <input id="input" use:setType={type} bind:value={value} />
</div>