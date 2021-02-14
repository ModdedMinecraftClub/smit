<style>
    #formhostcontainer{
        text-align: center;
    }

    #fields{
        border-bottom: 1px dashed #6B7280;
    }
</style>

<div id="formhostcontainer">
    <div class="smit-card-wide">
        <div id="fields">
            <slot name="fields"></slot>
        </div>
        <div id="buttons">
            <slot name="buttons"></slot>
        </div>
    </div>
</div>