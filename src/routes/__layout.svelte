<div id="svelte" class="container is-fluid my-5">
    <nav class="navbar is-dark" role="navigation">
        <div class="navbar-brand">
            <div class="navbar-item ml-5">
                <img
                    src="/favicon.png"
                    width="32"
                    height="32"
                    alt="Camera Lens by Lívia Goto from the Noun Project"
                />
            </div>
            <h1 class="title is-2 navbar-item">Lensflare</h1>
        </div>
        <div id="navbarBasicExample" class="navbar-menu">
            <div class="navbar-start">
                <a class="navbar-item" href="/"> Home </a>
                <a class="navbar-item" href="/post"> Post </a>
            </div>
        </div>
    </nav>

    <slot />
</div>
