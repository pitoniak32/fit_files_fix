#!/usr/bin/python3

from fitparse import FieldData

# enduro2
garmin_product = 4
manufacturer = "garmin"
file_path = '/home/davidpi/Downloads/fitfiletools.fit' 

fitfile = fitparse.FitFile(file_path)

for record in fitfile.get_messages():
    if record.name == 'file_id':
        # Access and modify device information
        record_fields = record.fields
        for field in record_fields:
            if field.name == 'manufacturer':
                setattr(field, 'value', 'g')

            # there is probably not already a field for this.
            #  <FieldData: garmin_product: 4341, def num: 2, type: garmin_product (uint16), raw value: 4341>
            if field.name == 'garmin_product':
                print(dir(field))
                setattr(field, 'name', 'garmin_product')
                setattr(field, 'value', 0)

        record.fields = record_fields
        print(record.fields)

print(dir(fitfile))

fitfile.close()
