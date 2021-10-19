<script lang="ts">
    import superagent from "superagent";
    import { text } from "svelte/internal";

    let url = "https://lensflare-api.mileswatson.workers.dev/posts";

    let request = superagent.get(url);
</script>

<div class="box is-dark">
    {#await request}
        <div class="notification is-info">
            <h1>Loading...</h1>
        </div>
    {:then res}
        <div class="columns is-multiline">
            <div class="column" />
            <div class="column is-6">
                {#each JSON.parse(res.text) as post}
                    <div class="card">
                        {#if post["type"] == "image"}
                            <div class="card-image">
                                <figure class="image">
                                    <img src={post["content"]} alt="Post" />
                                </figure>
                            </div>
                        {/if}
                        <div class="card-content">
                            <div class="media">
                                <div class="media-content">
                                    <p class="title is-4">{post["title"]}</p>
                                    <p class="subtitle is-6">
                                        {post["username"]}
                                    </p>
                                </div>
                            </div>

                            <div class="content">
                                {#if post["type"] == "text"}
                                    {post["content"]}
                                {:else if post["type"] == "link"}
                                    <a href={post["content"]}>Link</a>
                                {/if}
                            </div>
                        </div>
                    </div>
                    <br />
                {:else}
                    <div class="notification is-warning">
                        <h1>No posts!</h1>
                    </div>
                {/each}
            </div>
            <div class="column" />
        </div>
    {/await}
</div>
