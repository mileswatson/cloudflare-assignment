<script lang="ts">
    import superagent from "superagent";

    let url = "https://cloudflare-assignment.mileswatson.workers.dev/posts";

    let username = "";
    let title = "";
    let type = "text";
    let content = "";

    async function submit() {
        try {
            console.log(username, title, type, content);
            await superagent.get(url);
            await superagent.post(url).send({ username, title, type, content });
        } catch (e) {
            error = `Posting failed! ${e.text}`;
        }
    }

    let error = null;
</script>

<div class="box">
    <div class="field">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">Username</label>
        <div class="control has-icons-left has-icons-right">
            <input
                class="input"
                type="text"
                placeholder="Text input"
                bind:value={username}
            />
            <span class="icon is-small is-left">
                <i class="fas fa-user" />
            </span>
            <span class="icon is-small is-right">
                <i class="fas fa-check" />
            </span>
        </div>
    </div>

    <div class="field">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">Title</label>
        <div class="control">
            <input
                class="input"
                type="text"
                placeholder="Text input"
                bind:value={title}
            />
        </div>
    </div>

    <div class="field">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">Content Type</label>
        <div class="control">
            <div class="select">
                <select bind:value={type}>
                    <option value="text">Text</option>
                    <option value="link">Link</option>
                    <option value="image">Image Link</option>
                </select>
            </div>
        </div>
    </div>

    <div class="field">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">Content</label>
        <div class="control">
            <textarea
                bind:value={content}
                class="textarea"
                placeholder="Textarea"
            />
        </div>
    </div>

    <div class="field is-grouped">
        <div class="control">
            <button class="button is-link" on:click={submit}>Submit</button>
        </div>
        <div class="control">
            <button
                class="button is-link is-light"
                on:click={() => window.location.replace("/")}>Cancel</button
            >
        </div>
    </div>
</div>
