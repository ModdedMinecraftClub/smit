<script>
    import { metatags } from '@roxi/routify';
    metatags.title = 'Smit Index';
</script>

<style>
    .issue-card{
        border-radius: 5px;
        border: 1px solid grey;
        display: inline-block;
        width: 20em;
        padding: 25px;
        box-shadow: 0px 0px 5px #9CA3AF;
    }

    .issue-card-title{
        font-weight: bold;
        font-size: 18px;
        margin-bottom: 10px;
    }

    .issue-badges{
        border-top: 1px dashed #6B7280;
        border-bottom: 1px dashed #6B7280;
        padding-top: 5px;
        padding-bottom: 10px;
    }

    .issue-badge{
        display: inline-block;
        color: white;
        border-radius: 5px;
        padding: 8px;
        font-size: 12px;
        font-weight: bold;
        margin-right: 2.5px;
        margin-top: 5px;
        box-shadow: 0px 0px 5px #9CA3AF;
    }

    .issue-description{
        margin-top: 10px;
        padding: 3px;
    }

</style>

<div class="issue-card">
    <div class="issue-card-title">
        Example Issue
    </div>
    <div class="issue-badges">
        <div class="issue-badge" style="background-color: #DC2626">Requires Admin Assistance</div>
        <div class="issue-badge" style="background-color: #7C3AED;">Requires Moderator Assistance</div>
        <div class="issue-badge" style="background-color: #34D399;">Open</div>
    </div>
    <div class="issue-description">
        I want to do following to add tailwind configuration to rollup, but its throwing errors because I don'nt know to overwrite the config right. Could you help me please? My Code:
    </div>
</div>