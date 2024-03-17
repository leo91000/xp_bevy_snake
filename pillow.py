from PIL import Image

# Load your image
image_path = 'C:\\Users\\maill\\Pictures\\foods.png'  # Replace this with the path to your image
original_image = Image.open(image_path)

# Dimensions of each icon
icon_width, icon_height = 32, 32

# Number of icons in each dimension
num_columns, num_rows = 6, 2

# Split and save each icon
for row in range(num_rows):
    for col in range(num_columns):
        left = col * icon_width
        upper = row * icon_height
        right = left + icon_width
        lower = upper + icon_height

        # Define the bounding box for the current icon
        bounding_box = (left, upper, right, lower)

        # Crop the current icon out of the original image
        icon_image = original_image.crop(bounding_box)

        # Save the icon to a new file
        icon_image.save(f'icon_{row}_{col}.png')

