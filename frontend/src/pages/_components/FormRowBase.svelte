<script>
    export let title;
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
</style>

<div id="form-row">
    <label id="label">
        {title}
    </label>
    <slot/>
</div>