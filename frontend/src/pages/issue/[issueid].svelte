<script>
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
    <span class="issue-title" style="flex-grow: 1">Example Issue Title</span>
    <div class="smit-badge" style="background-color: #DC2626">Requires Admin Assistance</div>
    <div class="smit-badge" style="background-color: #7C3AED;">Requires Moderator Assistance</div>
    <div class="smit-badge" style="background-color: #34D399;">Open</div>
</div>

<div id="comments-container">
    <div class="smit-card">
        <div class="comment-author">john01dav</div>
        <p>
            I forgot i died on GT server (non-ironman), and reinstalled pack, clearing journeymap waypoints causing me to lose my grave with my items in it. is there a way to get the grave location/contents
        </p>
    </div>

    <div class="smit-card">
        <div class="comment-author">john01dav</div>
        <p>
            I forgot i died on GT server (non-ironman), and reinstalled pack, clearing journeymap waypoints causing me to lose my grave with my items in it. is there a way to get the grave location/contents
        </p>
    </div>

    <div class="smit-card">
        <div class="comment-author">john01dav</div>
        <p>
            I forgot i died on GT server (non-ironman), and reinstalled pack, clearing journeymap waypoints causing me to lose my grave with my items in it. is there a way to get the grave location/contents
        </p>
    </div>

    <div class="smit-card">
        <div class="comment-author">john01dav</div>
        <p>
            I forgot i died on GT server (non-ironman), and reinstalled pack, clearing journeymap waypoints causing me to lose my grave with my items in it. is there a way to get the grave location/contents
        </p>
    </div>

    <div class="smit-card">
        <div class="comment-author">john01dav</div>
        <p>
            I forgot i died on GT server (non-ironman), and reinstalled pack, clearing journeymap waypoints causing me to lose my grave with my items in it. is there a way to get the grave location/contents
        </p>
    </div>
</div>