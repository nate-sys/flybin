<div class="container">
	<form method="POST">
		<label for="username"> Username </label>
		<input type="text" name="username" id="username" />
		<label for="password">Password </label>
		<input type="password" name="password" id="password" />
		<button>Login</button>
	</form>
</div>

<style>
	.container {
		margin: 2rem;
        padding: 1rem;
        background: #1a1156;
	}
	form {
		display: flex;
		flex-direction: column;
        gap: .5ch;
	}
    input, button {
        appearance: none;
        border: none;
        padding: 1ch 1.5ch;
    }
</style>
