<script>
    import FormRowBase from './FormRowBase.svelte';

    export let title;
    export let type;
    export let value;

    function setType(node, type) {
        node.type = type;
    }

    value = "";
</script>

<style>
    .input{
        flex-grow: 1;
        min-width: 0;
        height: 32rem;
    }
</style>

<FormRowBase title={title}>
    <textarea class="input" bind:value={value}></textarea>
</FormRowBase>