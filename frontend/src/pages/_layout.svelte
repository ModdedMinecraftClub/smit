<style>
    #main-navbar{
        display: flex;
        flex-direction: row;
        align-items: center;
        height: 5rem;
        margin: 0px;
        padding: 0px 20px;
        box-shadow: 0px 0px 5px #6B7280;
        background-color: #F9FAFB;
        z-index: -1;
    }

    #navbar-brand{
        flex-grow: 1;
        font-size: 25px;
        font-weight: bold;
    }

    #main-container{
        padding: 16px;
    }
</style>

<div id="main-navbar">
    <div id="navbar-brand">
        <a href="/" style="text-decoration: none; color: inherit;">Smit</a>
    </div>
    <div id="navbar-credentials">
        <a href="/signup" class="smit-button" style="background-color: #2563EB">Sign up</a>
        <a href="/signin" class="smit-button" style="background-color: #059669">Sign in</a>
    </div>
</div>

<div id="main-container">
    <slot/>
</div>