<script lang="ts">
	import {
		Button,
		Card,
		CardBody,
		CardFooter,
		CardHeader,
		CardSubtitle,
		CardText,
		CardTitle,
		Container,
	} from 'sveltestrap';

	const handleClick = () => alert('I warned you!');

	export let name: string;
</script>

<main>
	<Container>
		<Card class="mb-3">
			<CardHeader>
				<CardTitle>Hello {name}!</CardTitle>
			</CardHeader>
			<CardBody>
				<CardSubtitle>Card subtitle</CardSubtitle>
				<CardText>
					Some quick example text to build on the card title and make
					up the bulk of the card's content.
				</CardText>
				<Button color="danger" on:click={handleClick}
					>Do Not Press</Button
				>
			</CardBody>
			<CardFooter>Footer</CardFooter>
		</Card>
	</Container>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
