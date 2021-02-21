<script>
    import {getFetchJson} from "../../ajax.js";
    import {unixTimestampHumanReadableLocal} from "../../utils.js";

    export let issueid;
</script>

<style>
    .comment-author{
        font-weight: bold;
        font-size: 18px;
        margin-bottom: 10px;
    }

    #issue-header{
        display: flex;
        flex-direction: row;
        align-items: center;
        flex-wrap: wrap;
    }

    .issue-title{
        font-weight: bold;
        font-size: 25px;
    }

    #comments-container{
        text-align: center;
        margin: 0px;
    }

    .smit-card{
        display: inline-block;
        width: calc(100% - 50px);
        text-align: left;
        margin: 20px 0px;
    }

    .smit-badge{
        margin-left: 10px;
    }
</style>

<div id="issue-header">
    <span class="issue-title" style="flex-grow: 1">
        {#await getFetchJson(`/api/issues/${issueid}`)}
            Loading...
        {:then metadata_json}
            {metadata_json.title}
        {:catch error}
            {error.message}
        {/await}
    </span>
    <div class="smit-badge" style="background-color: #DC2626">Requires Admin Assistance</div>
    <div class="smit-badge" style="background-color: #7C3AED;">Requires Moderator Assistance</div>
    <div class="smit-badge" style="background-color: #34D399;">Open</div>
</div>

<div id="comments-container">
    {#await getFetchJson(`/api/issues/${issueid}/comments`)}
        Loading...
    {:then comments_json}
        {#each comments_json as comment}
            <div class="smit-card">
                <div class="comment-author">{comment.username} ({unixTimestampHumanReadableLocal(comment.timestamp)})</div>
                <p>
                    {comment.contents}
                </p>
            </div>
        {/each}
    {:catch error}
        {error.message}
    {/await}
</div>