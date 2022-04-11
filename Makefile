TAG ?= latest
REGISTRY ?= localhost:5000
tailwind-watch:
	tailwindcss -w -c tailwind.config.js -o tailwind.css

serve:
	trunk serve

docker-image:
	docker build -t wordle-frontend:$(TAG) .                         
	docker tag wordle-frontend:$(TAG) $(REGISTRY)/wordle-frontend:$(TAG)
	docker push $(REGISTRY)/wordle-frontend:$(TAG)